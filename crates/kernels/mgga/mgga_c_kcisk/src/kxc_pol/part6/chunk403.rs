//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 403/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk403<F: Float>(t2042: F, t240: F, t2542: F, t2595: F, t2656: F, t2666: F, t802: F, t567: F, t116: F, t213: F, t172: F, t32: F, t5: F) -> (F, F, F, F) {
    let t2670 = t2542 - t2595 + t240 * (-t2042 * t2666 + t2656 * t802 - t2542 + t2595);
    let t2671 = t567 * t2670;
    let t2689 = t116 * t213;
    let t2849 = F::cast_from(0.14764770444444444444e-2_f64) * t5 * t172 * t32;
    (t2670, t2671, t2689, t2849)
}
