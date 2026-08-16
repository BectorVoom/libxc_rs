//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 844/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk844<F: Float>(t1378: F, t6460: F, t1375: F, t1843: F, t5215: F, t5321: F, t568: F, t6362: F, t6364: F, t6435: F, t6440: F, t1297: F, t1390: F, t193: F, t2486: F, t3701: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t3924: F, t533: F, t6324: F, t6329: F, t6330: F, t6347: F, t6399: F, t6400: F) -> (F, F, F) {
    let t6461 = t1378 * t6460;
    let t6463 = F::cast_from(2.0_f64) * t1375 * t6440 - t1375 * t6461 - F::cast_from(2.0_f64) * t1843 * t5215 - F::cast_from(2.0_f64) * t1843 * t5321 + t568 * t6362 + F::cast_from(2.0_f64) * t568 * t6364 + t568 * t6435;
    let t6467 = t1390 * t193 * t533 * t6463 - t193 * t3701 * t533 * t6324 + F::cast_from(3.0_f64) * t1297 * t193 * t6347 + F::cast_from(6.0_f64) * t193 * t3924 * t6330 - t2486 + t3819 + t3821 + t3823 + t3825 - t3832 - t3836 + t6329 - t6399 - t6400;
    (t6461, t6463, t6467)
}
