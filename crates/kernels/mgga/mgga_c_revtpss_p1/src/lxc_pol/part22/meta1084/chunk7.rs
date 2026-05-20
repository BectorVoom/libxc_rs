//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3932/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3932<F: Float>(t10416: F, t13426: F, t13435: F, t13521: F, t13532: F, t13540: F, t13544: F, t14310: F, t18163: F, t18232: F, t18242: F, t18245: F, t1847: F, t21658: F, t21882: F, t21891: F, t2322: F, t2372: F, t4248: F, t4254: F, t4297: F, t569: F, t5887: F, t5921: F, t651: F, t670: F, t75672: F, t7732: F) -> F {
    let t75676 = -F::new(4.0) * t21658 * t651 * t670 - F::new(4.0) * t10416 * t5887 - F::new(8.0) * t13426 * t4297 - F::new(8.0) * t13435 * t5887 - F::new(4.0) * t13521 * t4248 - F::new(8.0) * t13532 * t4248 - F::new(8.0) * t13532 * t7732 - F::new(8.0) * t13540 * t4248 - F::new(4.0) * t13544 * t4248 + F::new(2.0) * t14310 * t1847 - F::new(2.0) * t18163 * t5921 - F::new(4.0) * t18232 * t2322 - F::new(4.0) * t18242 * t4254 - F::new(2.0) * t18245 * t2372 - F::new(4.0) * t21882 * t2322 - F::new(8.0) * t21891 * t2322 + t569 * t75672;
    t75676
}
