//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1538/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1538<F: Float>(t12606: F, t998: F, t974: F, t10868: F, t1539: F, t248: F, t1041: F, t1009: F, t4552: F, t1011: F, t1019: F, t1615: F, t3131: F) -> (F, F, F, F, F) {
    let t14197 = t998 * t12606;
    let t14198 = t974 * t14197;
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    let t14206 = t14205 * t1011;
    let t14207 = t14206 * t1019;
    let t14211 = t1615 * t3131;
    (t14198, t14203, t14205, t14207, t14211)
}
