//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 671/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk671(t1985: f64, t3851: f64, t3140: f64, t7472: f64, t14039: f64, t3116: f64, t3128: f64, t14123: f64) -> (f64, f64, f64, f64, f64) {
    let t68427 = t1985 * t3851;
    let t68432 = t7472 * t3140;
    let t68438 = t14039 * t3116;
    let t68439 = t3128 * t68438;
    let t68440 = t68439 * t14123;
    (t68427, t68432, t68438, t68439, t68440)
}
