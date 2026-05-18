//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 299/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk299<F: Float>(t1263: F, t1272: F, t1268: F, t1275: F, t363: F, t306: F, t1217: F, t1221: F, t1225: F, t1280: F, t1289: F, t1290: F, t1292: F, t1304: F, t1307: F, t1313: F, t1317: F, t1321: F, t1322: F, t311: F) -> (F, F, F, F, F, F, F) {
    let t1325 = F::new(2.0) * t1263;
    let t1327 = F::new(2.0) / F::new(3.0) * t1272;
    let t1329 = t1325 - F::new(2.0) * t1268 + t1327 + F::new(2.0) * t1275;
    let t1330 = F::new(1.0) / t363;
    let t1331 = t1329 * t1330;
    let t1332 = t1331 * t306;
    let t1335 = -F::new(22.07984838129906) * t1217 + t1221 + t1225 + F::new(1.8805371096875316) * t1280 * t311 - t1289 - F::new(19.489173774580152) * t1290 * t1292 + F::new(19.489173774580152) * t1304 * t311 + F::new(3.7610742193750633) * t1307 * t1292 - t1313 + t1317 - t1321 - F::new(4.937333717448355) * t1322 * t1292 + F::new(4.937333717448355) * t1332 * t311;
    (t1325, t1327, t1329, t1330, t1331, t1332, t1335)
}
