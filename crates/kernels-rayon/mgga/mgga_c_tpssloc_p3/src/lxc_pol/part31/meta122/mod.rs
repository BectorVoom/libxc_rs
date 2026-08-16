//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta122 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk680;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta122(t135: f64, t999: f64, t973: f64, t2770: f64, t2978: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64, t221: f64, t2965: f64, t339: f64, t964: f64, t995: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3140, t3146, t3151, t3156, t3158, t3160) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk680(t135, t999, t973, t2770, t2978, t2775, t976, t1005, t1036, t221, t2965, t339);
        let (t3163, t3169, t3174, t3180, t3185, t3186) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk681(t964, t995, t1050, t225, t1053, t386, t68, t1057, t3112, t3032, t3127, t3031);
    (t3140, t3146, t3151, t3156, t3158, t3160, t3163, t3169, t3174, t3180, t3185, t3186)
}
