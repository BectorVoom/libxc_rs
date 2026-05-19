//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 793/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk793<F: Float>(t169: F, t171: F, t2629: F, t6272: F, t6276: F, t1650: F, zeta_threshold: F) -> (F, F) {
    let t170 = t169 <= zeta_threshold;
    let t6280 = piecewise3::<F>(t170, F::new(0.0), F::new(4.0) / F::new(9.0) * t2629 * t6272 + F::new(4.0) / F::new(3.0) * t171 * t6276);
    let t6281 = t1650 * t1650;
    (t6280, t6281)
}
