//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1370/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1370(t14001: f64, t15334: f64, t2409: f64, t36000: f64, t3959: f64, t14673: f64, t2503: f64, t13796: f64, t15167: f64, t3989: f64, t53539: f64, t11633: f64, t53710: f64, t56296: f64) -> (f64, f64, f64, f64, f64) {
    let t57542 = t14001 * t15334;
    let t57545 = t3959 * t2409 * t36000;
    let t57551 = t14673 * t2503;
    let t57555 = t3989 * t13796 * t53539 * t15167;
    let t57570 = t3989 * t53710 * t56296 * t11633;
    (t57542, t57545, t57551, t57555, t57570)
}
