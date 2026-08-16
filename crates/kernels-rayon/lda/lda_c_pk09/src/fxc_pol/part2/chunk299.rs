//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 299/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk299(t1263: f64, t1272: f64, t1268: f64, t1275: f64, t363: f64, t306: f64, t1217: f64, t1221: f64, t1225: f64, t1280: f64, t1289: f64, t1290: f64, t1292: f64, t1304: f64, t1307: f64, t1313: f64, t1317: f64, t1321: f64, t1322: f64, t311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1325 = 2.0_f64 * t1263;
    let t1327 = 2.0_f64 / 3.0_f64 * t1272;
    let t1329 = t1325 - 2.0_f64 * t1268 + t1327 + 2.0_f64 * t1275;
    let t1330 = 1.0_f64 / t363;
    let t1331 = t1329 * t1330;
    let t1332 = t1331 * t306;
    let t1335 = -22.07984838129906_f64 * t1217 + t1221 + t1225 + 1.8805371096875316_f64 * t1280 * t311 - t1289 - 19.489173774580152_f64 * t1290 * t1292 + 19.489173774580152_f64 * t1304 * t311 + 3.7610742193750633_f64 * t1307 * t1292 - t1313 + t1317 - t1321 - 4.937333717448355_f64 * t1322 * t1292 + 4.937333717448355_f64 * t1332 * t311;
    (t1325, t1327, t1329, t1330, t1331, t1332, t1335)
}
