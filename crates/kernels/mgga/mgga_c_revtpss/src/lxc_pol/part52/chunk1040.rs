//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1040/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1040<F: Float>(t1959: F, t31759: F, t32426: F, t32430: F, t32434: F, t32437: F, t32438: F, t32439: F, t32441: F, t32445: F, t32450: F, t32456: F, t7073: F, t7079: F, t7083: F, t8645: F, t8649: F, t8652: F) -> F {
    let t32457 = F::new(0.57119737665102352616e0) * t32426 * t8652 + F::new(0.57119737665102352616e0) * t8649 * t32430 + F::new(0.17347256376410398924e1) * t32434 * t7073 - t32437 + t32438 - t32439 + F::new(0.57119737665102352616e0) * t8649 * t32441 - F::new(0.17135921299530705785e1) * t8649 * t32445 + F::new(0.8673628188205199462e0) * t32434 * t7079 - F::new(0.8673628188205199462e0) * t32450 * t1959 - F::new(0.56468933516960933999e-3) * t31759 - F::new(0.8673628188205199462e0) * t8645 * t7083 + t32456;
    t32457
}
