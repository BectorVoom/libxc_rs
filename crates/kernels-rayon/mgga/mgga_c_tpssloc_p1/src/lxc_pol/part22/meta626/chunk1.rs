//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2161/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2161(t53984: f64, t40281: f64, t5303: f64, t5247: f64, t820: f64, t12250: f64, t1824: f64, t16060: f64, t3789: f64, t12384: f64, t5234: f64, t5293: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53985 = 35.0_f64 / 72.0_f64 * t53984;
    let t53997 = t40281 * t5303;
    let t53998 = 119.0_f64 / 1152.0_f64 * t53997;
    let t54013 = t5247 * t820;
    let t54014 = t1824 * t12250;
    let t54023 = t16060 * t3789;
    let t54042 = t5234 * t12384;
    let t54047 = t40281 * t5293;
    (t53985, t53998, t54013, t54014, t54023, t54042, t54047)
}
