//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 476/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk476<F: Float>(t1417: F, t2615: F, t2621: F, t392: F, t1425: F, t306: F, t2606: F, t1225: F, t1289: F, t1313: F, t1317: F, t1321: F, t1322: F, t1341: F, t1371: F, t1405: F, t1451: F, t2513: F, t2525: F, t2531: F, t2611: F, t297: F, t311: F) -> (F, F, F, F, F) {
    let t2624 = t2615 * t392 - t1417 * t2621 / F::cast_from(2.0_f64);
    let t2625 = t2624 * t1425;
    let t2626 = t2625 * t306;
    let t2629 = t2606 * t306;
    let t2634 = t1225 - t1289 - t1313 + t1317 - t1321 + t1341 - t1371 + t1405 - F::cast_from(22.07984838129906_f64) * t2525 - F::cast_from(2.9824072957409817_f64) * t2531 * t1451 + t297 * t2611 - F::cast_from(2.2140749178833072_f64) * t2626 * t311 + F::cast_from(19.489173774580152_f64) * t2629 * t311 - F::cast_from(4.937333717448355_f64) * t1322 * t2513;
    (t2624, t2625, t2626, t2629, t2634)
}
