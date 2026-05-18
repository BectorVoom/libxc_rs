//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1022/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1022<F: Float>(t21125: F, t6151: F, t18183: F, t21130: F, t18187: F, t21134: F, t1595: F, t7403: F, t12842: F, t18093: F, t18192: F, t23115: F, t23119: F, t23123: F, t23126: F, t23130: F, t4439: F, t6152: F, t6156: F, t6160: F) -> F {
    let t23133 = t6151 * t21125;
    let t23136 = t18183 * t21130;
    let t23139 = t18187 * t21134;
    let t23149 = t7403 * t1595;
    let t23151 = -t4439 * t23115 / F::new(288.0) + t4439 * t23119 / F::new(144.0) + t4439 * t23123 / F::new(288.0) + t4439 * t23126 / F::new(96.0) - t4439 * t23130 / F::new(432.0) - t4439 * t23133 / F::new(72.0) + F::new(7.0) / F::new(1296.0) * t4439 * t23136 - t4439 * t23139 / F::new(108.0) + t18192 * t6156 / F::new(108.0) + t18192 * t6160 / F::new(54.0) - t18192 * t6152 / F::new(81.0) + t12842 / F::new(864.0) - t18093 + F::new(11.0) / F::new(648.0) * t23149;
    t23151
}
