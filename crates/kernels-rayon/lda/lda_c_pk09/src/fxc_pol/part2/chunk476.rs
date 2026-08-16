//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 476/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk476(t1417: f64, t2615: f64, t2621: f64, t392: f64, t1425: f64, t306: f64, t2606: f64, t1225: f64, t1289: f64, t1313: f64, t1317: f64, t1321: f64, t1322: f64, t1341: f64, t1371: f64, t1405: f64, t1451: f64, t2513: f64, t2525: f64, t2531: f64, t2611: f64, t297: f64, t311: f64) -> (f64, f64, f64, f64, f64) {
    let t2624 = t2615 * t392 - t1417 * t2621 / 2.0_f64;
    let t2625 = t2624 * t1425;
    let t2626 = t2625 * t306;
    let t2629 = t2606 * t306;
    let t2634 = t1225 - t1289 - t1313 + t1317 - t1321 + t1341 - t1371 + t1405 - 22.07984838129906_f64 * t2525 - 2.9824072957409817_f64 * t2531 * t1451 + t297 * t2611 - 2.2140749178833072_f64 * t2626 * t311 + 19.489173774580152_f64 * t2629 * t311 - 4.937333717448355_f64 * t1322 * t2513;
    (t2624, t2625, t2626, t2629, t2634)
}
