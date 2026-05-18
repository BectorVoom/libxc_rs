//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1158/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1158<F: Float>(t10102: F, t10109: F, t10113: F, t10115: F, t10131: F, t10133: F, t10137: F, t10141: F, t10144: F, t13558: F, t13564: F, t13578: F, t13584: F, t13590: F, t13592: F, t13594: F, t13596: F, t13598: F, t13600: F) -> F {
    let t19534 = t13558 - t13564 + F::new(0.23911438650126355246e-1) * t10102 - F::new(0.31077233446777841256e-3) * t10109 + F::new(0.18736e-1) * t13578 - F::new(0.52833333333333333332e-2) * t13584 - F::new(0.4705225e-4) * t13590 + F::new(0.70444444444444444443e-2) * t13592 + F::new(0.78420416666666666667e-4) * t13594 - F::new(0.21858666666666666667e-1) * t13596 + t10113 - t10115 + F::new(0.47822877300252710492e-1) * t13598 - F::new(0.62154466893555682512e-3) * t13600 - F::new(0.10929333333333333333e-1) * t10131 + F::new(0.35222222222222222222e-2) * t10133 + t10137 + t10141 - t10144;
    t19534
}
