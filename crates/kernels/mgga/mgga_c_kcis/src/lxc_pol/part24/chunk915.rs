//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 915/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk915<F: Float>(t1092: F, t19663: F, t13271: F, t13278: F, t13302: F, t13303: F, t13305: F, t13308: F, t13312: F, t19628: F, t19633: F, t19636: F, t19640: F, t19642: F, t19645: F, t19648: F, t19651: F, t19658: F, t19661: F, t9552: F) -> (F, F) {
    let t19664 = t1092 * t19663;
    let t19670 = -F::new(0.33163888888888888888e-2) * t19628 - F::new(0.33163888888888888888e-2) * t19633 + F::new(0.16581944444444444444e-2) * t19636 + F::new(0.33163888888888888888e-2) * t19640 - F::new(0.33163888888888888888e-2) * t19642 + F::new(0.13265555555555555555e-1) * t19645 + F::new(0.33163888888888888888e-2) * t19648 + F::new(0.16581944444444444444e-2) * t19651 + F::new(0.16581944444444444444e-2) * t19658 - F::new(0.49745833333333333332e-2) * t19661 + t13271 + F::new(0.99491666666666666664e-2) * t19664 - F::new(0.36848765432098765431e-3) * t9552 - t13278 + t13302 + F::new(0.11054629629629629629e-2) * t13303 + F::new(0.88437037037037037035e-2) * t13305 + t13308 - F::new(0.58958024691358024688e-2) * t13312;
    (t19664, t19670)
}
