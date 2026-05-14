//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1048/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1048<F: Float>(t10047: F, t8456: F, t2344: F, t3849: F, t10122: F, t8428: F, t926: F, t10076: F, t8435: F, t10244: F, t2380: F, t6475: F, t3214: F, t8363: F, t204: F, t648: F, t9795: F) -> (F, F, F, F, F, F, F) {
    let t27155 = t10047 * t8456;
    let t27175 = t3849 * t2344;
    let t27178 = t8428 * t926 * t10122;
    let t27181 = t8435 * t926 * t10076;
    let t27232 = t2380 * t6475 * t10244;
    let t27234 = t3214 * t8363;
    let t27262 = t204 * t648 * t9795;
    (t27155, t27175, t27178, t27181, t27232, t27234, t27262)
}
