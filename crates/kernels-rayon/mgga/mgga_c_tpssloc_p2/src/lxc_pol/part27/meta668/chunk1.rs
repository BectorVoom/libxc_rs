//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2356/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356(t22480: f64, t4028: f64, t12545: f64, t12734: f64, t1774: f64, t22461: f64, t22600: f64, t2314: f64, t2364: f64, t24999: f64, t25965: f64, t4077: f64, t6517: f64, t7472: f64, t91578: f64, t91580: f64, t91582: f64, t91585: f64, t91587: f64, t91589: f64, t91591: f64, t91593: f64, t91602: f64, t91606: f64, t91608: f64, t91610: f64) -> f64 {
    let t91612 = 2.0_f64 * t4028 * t22480;
    let t91617 = -4.0_f64 * t12545 * t6517 - 4.0_f64 * t12734 * t7472 - 2.0_f64 * t1774 * t22600 - 4.0_f64 * t22461 * t4077 - 4.0_f64 * t2314 * t25965 - 2.0_f64 * t2364 * t24999 - t91578 - t91580 + t91582 + t91585 - t91587 - t91589 - t91591 - t91593 - t91602 - t91606 - t91608 - t91610 - t91612;
    t91617
}
