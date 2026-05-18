//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 950/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk950<F: Float>(t9062: F, t9066: F, t9150: F, t9152: F, t9155: F, t9158: F, t9163: F, t9166: F, t9168: F, t9170: F, t9173: F, t9176: F, t9179: F, t9182: F) -> F {
    let t9311 = -t9062 / F::new(8.0) - F::new(3.0) / F::new(4.0) * t9066 + t9150 / F::new(8.0) - F::new(3.0) / F::new(8.0) * t9152 + F::new(3.0) / F::new(32.0) * t9155 + t9158 / F::new(64.0) + F::new(15.0) / F::new(8.0) * t9163 - F::new(3.0) / F::new(2.0) * t9166 - F::new(3.0) / F::new(4.0) * t9168 + F::new(3.0) / F::new(64.0) * t9170 + F::new(3.0) / F::new(4.0) * t9173 - t9176 / F::new(64.0) + F::new(3.0) / F::new(8.0) * t9179 - F::new(3.0) / F::new(8.0) * t9182;
    t9311
}
