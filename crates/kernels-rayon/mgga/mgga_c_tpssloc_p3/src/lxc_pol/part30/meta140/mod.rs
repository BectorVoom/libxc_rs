//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk769;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta140(t1317: f64, t3726: f64, t2566: f64, t535: f64, t795: f64, t154: f64, t557: f64, t205: f64, t1314: f64, t792: f64, t118: f64, t1307: f64, t794: f64, t116: f64, t534: f64, t212: f64, t2586: f64, t1324: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk769(t1317, t3726, t2566, t535, t795, t154, t557);
        let (t3733, t3739, t3741, t3742, t3748, t3749, t3751, t3758) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk770(t205, t3732, t1314, t792, t118, t1307, t794, t116, t534, t212, t2586, t1324, t225);
    (t3727, t3731, t3732, t3733, t3739, t3741, t3742, t3748, t3749, t3751, t3758)
}
