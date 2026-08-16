//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 716/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk716<F: Float>(t1324: F, t225: F, t2600: F, t541: F, t1329: F, t3726: F, t119: F, t3734: F, t210: F, t3719: F, t3752: F, t554: F) -> (F, F, F, F, F, F, F) {
    let t3758 = t1324 * t225;
    let t3762 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2600 * t541;
    let t3763 = t3726 * t1329;
    let t3765 = t119 * t3734;
    let t3766 = t210 * t3765;
    let t3770 = t210 * t119 * t3719;
    let t3773 = t3752 * t225;
    let t3774 = t3773 * t554;
    (t3758, t3762, t3763, t3766, t3770, t3773, t3774)
}
