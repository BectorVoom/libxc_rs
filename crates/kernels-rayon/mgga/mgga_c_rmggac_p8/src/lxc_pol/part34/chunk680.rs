//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 680/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk680(t14025: f64, t35311: f64, t1965: f64, t68522: f64, t13850: f64, t1977: f64, t13858: f64, t2186: f64, t14286: f64, t352: f64, t262: f64, t8620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68651 = t14025 * t35311;
    let t68658 = t1965 * t68522;
    let t68660 = t1977 * t68658 * t13850;
    let t68669 = t2186 * t13858;
    let t68684 = t14286 * t352;
    let t68685 = t262 * t68684;
    let t68686 = t8620 * t68685;
    (t68651, t68658, t68660, t68669, t68684, t68685, t68686)
}
