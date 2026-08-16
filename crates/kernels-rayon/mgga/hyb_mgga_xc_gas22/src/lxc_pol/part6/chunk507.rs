//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 507/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk507(t7: f64, t2159: f64, t220: f64, t2337: f64, t291: f64, t771: f64, t861: f64, t909: f64, t314: f64, t1832: f64, t319: f64, t98: f64, t322: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t2341 = piecewise3(t9, 0.0_f64, t2159 * t291 / 2.0_f64 + t771 * t861 + t220 * t2337 / 2.0_f64);
    let t2345 = 1.0_f64 / t909;
    let t2350 = t314 * t314;
    let t2351 = t319 * t1832;
    let t2353 = 1.0_f64 / t98 / t2351;
    let t2355 = t322 * t322;
    (t2341, t2345, t2350, t2353, t2355)
}
