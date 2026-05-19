//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 879/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk879<F: Float>(t5560: F, t5859: F, t7332: F, t7387: F, t7390: F, t9178: F, t9180: F, t9185: F, t9189: F, t9192: F, t9196: F, t9200: F) -> F {
    let t9492 = F::new(0.31558125e0) * t9178 + F::new(0.6311625e0) * t9180 - t5859 + F::cast_from(0.34731666666666666666e0_f64) * t5560 + F::cast_from(0.69463333333333333333e0_f64) * t7332 - t7387 - t7390 - F::new(0.20839e0) * t9185 + F::new(0.62517e0) * t9189 - F::new(0.20839e0) * t9192 + F::new(0.312585e0) * t9196 + F::new(0.312585e0) * t9200;
    t9492
}
