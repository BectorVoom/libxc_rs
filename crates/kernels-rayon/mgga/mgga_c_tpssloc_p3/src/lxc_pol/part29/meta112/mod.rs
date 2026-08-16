//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk686;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk687;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta112(t2528: f64, t761: f64, t172: f64, t753: f64, t763: f64, t2504: f64, t739: f64, t746: f64, t40: f64, t52: f64, t718: f64, t751: f64, t2244: f64, t2250: f64, t75: f64, t767: f64, t771: f64, t78: f64, zeta_threshold: f64, t15: f64, t60: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2530, t2531, t2532, t2533, t2535) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk686(t2528, t761, t172, t753, t763, t2504, t739, t746);
        let (t2537, t2538, t2539, t2553) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk687(t40, t52, t2535, t761, t718, t751, t2244, t2250, t75, t767, t771, t78, zeta_threshold);
        let (t2558, t2559) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk688(t15, t60, t59);
    (t2530, t2531, t2532, t2533, t2535, t2537, t2538, t2539, t2553, t2558, t2559)
}
