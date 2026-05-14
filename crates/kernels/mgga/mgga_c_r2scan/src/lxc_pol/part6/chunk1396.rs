//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1396/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1396<F: Float>(t5917: F, t7824: F, t2758: F, t5418: F, t2483: F, t625: F, t1764: F, t1768: F, t21276: F, t21279: F, t21281: F, t21283: F, t21285: F, t21287: F, t21292: F, t21295: F, t26442: F) -> (F,) {
    let t26444 = t7824 * t5917;
    let t26446 = t2758 * t5418;
    let t26448 = t2483 * t625;
    let t26449 = t26448 * t1764;
    let t26450 = 0.65061487801810439052e-1 * t26449;
    let t26451 = t26448 * t1768;
    let t26452 = 0.96319466275353142157e0 * t26451;
    let t26457 = t21276 - 0.43374325201206959369e-1 * t26442 + 0.64212977516902094772e0 * t26444 - 0.62254000682014814813e-2 * t26446 + t26450 - t26452 - t21279 - 0.96319466275353142158e0 * t21281 + 0.19518446340543131716e0 * t21283 + 0.65061487801810439052e-1 * t21285 - 0.14447919941302971324e1 * t21287 + t21292 - t21295;
    (t26457,)
}
