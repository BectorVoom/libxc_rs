//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 472/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk472<F: Float>(t2341: F, t2352: F, t2354: F, t2358: F, t117: F, t118: F, t123: F, t125: F, t1328: F, t1330: F, t1333: F, t1337: F, t1341: F, t1345: F, t1349: F, t1352: F, t1356: F, t1360: F, t1363: F, t1799: F, t2323: F, t2327: F, t2331: F, t2338: F) -> (F, F) {
    let t2360 = t2341 + t2352 + t2354 + t2358;
    let t2365 = -t1328 + F::cast_from(0.031505407223141116_f64) * t1330 + t1333 + t1337 + F::cast_from(0.031505407223141116_f64) * t2323 - F::cast_from(0.031505407223141116_f64) * t1799 * t118 - F::cast_from(0.031505407223141116_f64) * t2327 - F::cast_from(0.001975389032890948_f64) * t2331 - F::cast_from(0.031505407223141116_f64) * t1341 - t1349 - t1352 - F::cast_from(0.001975389032890948_f64) * t1345 - t1356 - t1360 + F::cast_from(0.008980675507690957_f64) * t1363 + F::cast_from(0.008980675507690957_f64) * t2338 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * t2360 * t117;
    (t2360, t2365)
}
