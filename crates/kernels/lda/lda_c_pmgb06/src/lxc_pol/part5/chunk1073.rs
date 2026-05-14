//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1073/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1073<F: Float>(t20283: F, t20284: F, t20285: F, t20288: F, t20293: F, t20296: F, t20299: F, t20302: F, t20305: F, t20308: F, t20311: F, t20314: F, t18257: F, t18259: F, t20317: F, t20319: F, t20321: F, t20323: F, t20324: F, t20325: F, t20328: F, t20330: F, t20332: F) -> (F, F) {
    let t21982 = -t20283 - t20284 + t20285 + t20288 + t20293 - t20296 - t20299 + t20302 + t20305 - t20308 - t20311 + t20314;
    let t21987 = -t20317 - t20319 - t20321 + t20323 + t20324 + t20325 + 2.0 / 3.0 * t18257 + 4.0 / 3.0 * t18259 - t20328 - t20330 - t20332;
    (t21982, t21987)
}
