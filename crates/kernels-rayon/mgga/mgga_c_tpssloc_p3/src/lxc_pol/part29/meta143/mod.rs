//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk794;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta143(t2244: f64, t3146: f64, t974: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64, t221: f64, t2965: f64, t339: f64, t964: f64, t995: f64, t1000: f64, t1020: f64, t1025: f64, t1046: f64, t2955: f64, t2960: f64, t3109: f64, t3114: f64, t3117: f64, t3123: f64, t3130: f64, t3134: f64, t3140: f64, t3143: f64, t350: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3147, t3148, t3152, t3153, t3156, t3158, t3160, t3163) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk794(t2244, t3146, t974, t2775, t976, t1005, t1036, t221, t2965, t339, t964, t995);
        let t3165 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk795(t1000, t1020, t1025, t1046, t2955, t2960, t3109, t3114, t3117, t3123, t3130, t3134, t3140, t3143, t3148, t3153, t3156, t3160, t3163, t350, t973);
    (t3147, t3152, t3156, t3158, t3160, t3163, t3165)
}
