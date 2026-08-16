//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 902/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk902<F: Float>(t13580: F, t17923: F, t1127: F, t13654: F, t3724: F, t17856: F, t17877: F, t17883: F, t17891: F, t17896: F, t17900: F, t17904: F, t17908: F, t17912: F, t17916: F, t17919: F, t2387: F, t3723: F, t3759: F, t3766: F, t3767: F, t3789: F, t3790: F, t3817: F, t678: F, t680: F, t709: F, t9533: F) -> F {
    let t17924 = t13580 * t17923;
    let t17928 = t3724 * t13654 * t1127;
    let t17931 = -F::cast_from(0.40559281352147498558e-4_f64) * t17877 * t17856 + F::cast_from(4.0_f64) * t3789 * t3790 * t3817 + F::cast_from(2.0_f64) * t3789 * t17883 * t709 - F::cast_from(4.0_f64) * t3766 * t3767 * t3817 - F::cast_from(2.0_f64) * t3766 * t17891 + F::cast_from(0.19365723406274399941e-3_f64) * t678 * t17896 - F::cast_from(0.19365723406274399941e-3_f64) * t2387 * t17900 + F::cast_from(0.38731446812548799882e-3_f64) * t678 * t17904 + F::cast_from(0.11627450473218896e-1_f64) * t2387 * t17908 - F::cast_from(0.23254900946437792e-1_f64) * t9533 * t17912 - F::cast_from(0.23254900946437792e-1_f64) * t3759 * t17916 - F::cast_from(0.46509801892875584e-1_f64) * t3759 * t680 * t17919 + F::cast_from(0.27039520901431665705e-3_f64) * t3723 * t17924 - F::cast_from(0.13519760450715832853e-3_f64) * t3723 * t17928;
    t17931
}
