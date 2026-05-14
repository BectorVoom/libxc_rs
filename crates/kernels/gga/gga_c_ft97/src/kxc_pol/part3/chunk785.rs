//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 785/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk785<F: Float>(t25: F, t5025: F, t3762: F, t5005: F, t1113: F, t3751: F, t3725: F, t13580: F, t1127: F, t13654: F, t3724: F, t17856: F, t17877: F, t17883: F, t17891: F, t17896: F, t17900: F, t17904: F, t17908: F, t2387: F, t3723: F, t3759: F, t3766: F, t3767: F, t3789: F, t3790: F, t3817: F, t678: F, t680: F, t709: F, t9533: F) -> (F,) {
    let t17911 = t5025 * t25;
    let t17912 = t17911 * t3762;
    let t17915 = t5005 * t25;
    let t17916 = t17915 * t3762;
    let t17919 = t3751 * t1113;
    let t17923 = t3725 * t1113;
    let t17924 = t13580 * t17923;
    let t17928 = t3724 * t13654 * t1127;
    let t17931 = -0.40559281352147498558e-4 * t17877 * t17856 + 4.0 * t3789 * t3790 * t3817 + 2.0 * t3789 * t17883 * t709 - 4.0 * t3766 * t3767 * t3817 - 2.0 * t3766 * t17891 + 0.19365723406274399941e-3 * t678 * t17896 - 0.19365723406274399941e-3 * t2387 * t17900 + 0.38731446812548799882e-3 * t678 * t17904 + 0.11627450473218896e-1 * t2387 * t17908 - 0.23254900946437792e-1 * t9533 * t17912 - 0.23254900946437792e-1 * t3759 * t17916 - 0.46509801892875584e-1 * t3759 * t680 * t17919 + 0.27039520901431665705e-3 * t3723 * t17924 - 0.13519760450715832853e-3 * t3723 * t17928;
    (t17931,)
}
