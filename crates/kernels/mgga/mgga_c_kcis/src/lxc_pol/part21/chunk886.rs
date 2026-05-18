//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 886/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk886<F: Float>(t13480: F, t345: F, t13462: F, t1154: F, t13467: F, t10113: F, t10115: F, t10131: F, t10133: F, t10137: F, t10141: F, t10144: F, t104: F, t13578: F, t13584: F, t13590: F, t13592: F, t13594: F, t13596: F, t13598: F, t13600: F, t13602: F) -> F {
    let t13605 = t345 * t13480;
    let t13608 = t345 * t13462;
    let t13611 = t1154 * t13467;
    let t13616 = F::new(0.39814e-1) * t13578 - F::new(0.10038333333333333333e-1) * t13584 - F::new(0.77300125e-4) * t13590 + F::new(0.35222222222222222221e-2) * t13592 + F::new(0.39210208333333333333e-4) * t13594 - F::new(0.10929333333333333333e-1) * t13596 + t10113 - t10115 + F::new(0.23911438650126355246e-1) * t13598 - F::new(0.31077233446777841256e-3) * t13600 + F::new(0.7026e-2) * t104 * t13602 - F::new(0.7026e-2) * t104 * t13605 + F::new(0.1171e-2) * t104 * t13608 + F::new(0.78066666666666666667e-3) * t104 * t13611 - F::new(0.21858666666666666666e-1) * t10131 + F::new(0.70444444444444444443e-2) * t10133 + t10137 + t10141 - t10144;
    t13616
}
