//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 864/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk864<F: Float>(t1692: F, t2046: F, t2050: F, t31: F, t2604: F, t8413: F, t3928: F, t5187: F, t645: F, t4044: F, t5194: F, t1971: F, t236: F, t5704: F, t7365: F, t35331: F, t5700: F) -> (F, F, F, F, F, F) {
    let t41667 = t2046 * t2050 * t1692 * t31;
    let t41669 = t2604 * t8413;
    let t41672 = t3928 * t645 * t5187;
    let t41675 = t4044 * t645 * t5194;
    let t41690 = t7365 * t1971 * t236 * t5704;
    let t41694 = t35331 * t1971 * t236 * t5700;
    (t41667, t41669, t41672, t41675, t41690, t41694)
}
