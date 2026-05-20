//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1648/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1648<F: Float>(t240: F, t3252: F, t11145: F, t141: F, t11169: F, t930: F, t11158: F, t11162: F, t11167: F, t11316: F, t11319: F, t11322: F, t11326: F, t11329: F, t11332: F, t11334: F, t11338: F, t11339: F) -> (F, F, F, F, F, F) {
    let t11341 = t240 * t3252;
    let t11342 = t11341 * t11145;
    let t11343 = t141 * t11342;
    let t11345 = t930 * t11169;
    let t11346 = t141 * t11345;
    let t11349 = F::new(0.3071625e0) * t11316 - F::cast_from(0.82156666666666666668e-1_f64) * t11319 + F::cast_from(0.49293999999999999999e0_f64) * t11322 + F::new(0.17938e1) * t11167 - F::cast_from(0.59793333333333333333e0_f64) * t11158 - F::cast_from(0.32862666666666666666e0_f64) * t11326 + F::cast_from(0.16431333333333333333e0_f64) * t11329 - F::cast_from(0.49293999999999999999e0_f64) * t11332 - t11334 - t11338 + F::cast_from(0.5477111111111111111e-1_f64) * t11339 - F::cast_from(0.36514074074074074075e-1_f64) * t11343 - F::cast_from(0.82156666666666666667e-1_f64) * t11346 - F::new(0.17938e1) * t11162;
    (t11341, t11342, t11343, t11345, t11346, t11349)
}
