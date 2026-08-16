//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 921/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk921(t7: f64, t7889: f64, t7918: f64, t8176: f64, t8219: f64, t1291: f64, t1815: f64, t136: f64, t154: f64, t3188: f64, t157: f64, t160: f64, t163: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t8222 = piecewise3(t9, 0.0_f64, t7889 + t7918 + t8176 + t8219);
    let t8223 = t1815 * t1291;
    let t8224 = t136 * t8223;
    let t8226 = t154 * t3188;
    let t8231 = t157 * t3188;
    let t8236 = t160 * t3188;
    let t8241 = t163 * t3188;
    (t8222, t8223, t8224, t8226, t8231, t8236, t8241)
}
