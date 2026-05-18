//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1030/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1030<F: Float>(t13244: F, t1355: F, t19182: F, t2083: F, t25623: F, t306: F, t30605: F, t30616: F, t30916: F, t30938: F, t3599: F, t5687: F, t7757: F, t7764: F) -> F {
    let t30941 = F::new(3.0) / F::new(16.0) * t13244 * t30616 - F::new(3.0) / F::new(8.0) * t19182 * t7757 - F::new(3.0) / F::new(8.0) * t3599 * t30916 + F::new(3.0) / F::new(4.0) * t25623 * t2083 + F::new(3.0) / F::new(4.0) * t5687 * t7764 + t1355 * t30605 / F::new(4.0) + t306 * t30938 / F::new(2.0);
    t30941
}
