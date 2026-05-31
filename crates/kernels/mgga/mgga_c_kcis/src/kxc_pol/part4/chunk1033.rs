//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1033/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1033<F: Float>(t12938: F, t629: F, t2651: F, t908: F, t2791: F, t838: F, t169: F, t2628: F, t174: F, t2640: F, t2792: F, t2627: F) -> (F, F, F, F, F, F, F) {
    let t12939 = F::cast_from(1.0_f64) / t12938;
    let t12940 = t629 * t12939;
    let t12998 = t2651 * t908;
    let t13000 = t838 * t2791;
    let t13003 = F::cast_from(1.0_f64) / t2628 / t169;
    let t13014 = F::cast_from(1.0_f64) / t2640 / t174;
    let t13031 = F::cast_from(3.0_f64) * t2792;
    let t13034 = F::cast_from(3.0_f64) * t2627;
    (t12940, t12998, t13000, t13003, t13014, t13031, t13034)
}
