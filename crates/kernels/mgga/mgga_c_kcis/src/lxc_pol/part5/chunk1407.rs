//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1407/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1407<F: Float>(t1599: F, t1612: F, t18223: F, t23158: F, t23174: F, t23178: F, t23182: F, t23186: F, t23192: F, t23194: F, t23200: F, t23208: F, t23211: F, t23213: F, t6141: F, t6179: F, t6185: F) -> F {
    let t23215 = t23174 / F::new(1296.0) - t1599 * t23178 / F::new(32.0) + t1599 * t23182 / F::new(48.0) + t1599 * t23186 / F::new(576.0) - t6141 * t6179 / F::new(18.0) - t23192 / F::new(864.0) - t23194 / F::new(324.0) - t18223 / F::new(432.0) - t1599 * t23200 / F::new(192.0) - F::new(11.0) / F::new(216.0) * t23158 * t1612 + t6141 * t6185 / F::new(36.0) - t23208 / F::new(576.0) + t23211 / F::new(288.0) + t23213 / F::new(108.0);
    t23215
}
