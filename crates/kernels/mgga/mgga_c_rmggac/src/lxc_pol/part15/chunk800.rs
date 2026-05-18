//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 800/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk800<F: Float>(t1347: F, t2406: F, t16156: F, t9096: F, t8812: F, t7269: F, t8368: F, t7494: F, t8537: F, t1587: F, t2084: F, t2134: F, t27: F) -> (F, F, F, F, F, F) {
    let t38976 = t1347 * t2406;
    let t38986 = t16156 * t9096;
    let t38998 = t16156 * t8812;
    let t39023 = t8368 * t7269;
    let t39024 = F::new(0.18183107769496894486e-1) * t39023;
    let t39025 = t7494 * t8537;
    let t39031 = t2134 * t27 * t2084 * t1587;
    (t38976, t38986, t38998, t39024, t39025, t39031)
}
