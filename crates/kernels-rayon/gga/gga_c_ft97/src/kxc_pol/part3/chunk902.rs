//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 902/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk902(t13580: f64, t17923: f64, t1127: f64, t13654: f64, t3724: f64, t17856: f64, t17877: f64, t17883: f64, t17891: f64, t17896: f64, t17900: f64, t17904: f64, t17908: f64, t17912: f64, t17916: f64, t17919: f64, t2387: f64, t3723: f64, t3759: f64, t3766: f64, t3767: f64, t3789: f64, t3790: f64, t3817: f64, t678: f64, t680: f64, t709: f64, t9533: f64) -> f64 {
    let t17924 = t13580 * t17923;
    let t17928 = t3724 * t13654 * t1127;
    let t17931 = -0.40559281352147498558e-4_f64 * t17877 * t17856 + 4.0_f64 * t3789 * t3790 * t3817 + 2.0_f64 * t3789 * t17883 * t709 - 4.0_f64 * t3766 * t3767 * t3817 - 2.0_f64 * t3766 * t17891 + 0.19365723406274399941e-3_f64 * t678 * t17896 - 0.19365723406274399941e-3_f64 * t2387 * t17900 + 0.38731446812548799882e-3_f64 * t678 * t17904 + 0.11627450473218896e-1_f64 * t2387 * t17908 - 0.23254900946437792e-1_f64 * t9533 * t17912 - 0.23254900946437792e-1_f64 * t3759 * t17916 - 0.46509801892875584e-1_f64 * t3759 * t680 * t17919 + 0.27039520901431665705e-3_f64 * t3723 * t17924 - 0.13519760450715832853e-3_f64 * t3723 * t17928;
    t17931
}
