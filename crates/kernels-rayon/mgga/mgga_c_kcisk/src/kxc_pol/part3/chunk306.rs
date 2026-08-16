//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 306/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk306(t1341: f64, t1440: f64, t1415: f64, t1411: f64, t1299: f64, t470: f64, t468: f64, t415: f64, t382: f64, t394: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1441 = t1341 * t1440;
    let t1442 = t1415 * t1441;
    let t1443 = t1411 * t1442;
    let t1445 = sigma0 * t1299;
    let t1446 = t1445 * t470;
    let t1447 = t468 * t1446;
    let t1448 = t415 * t1447;
    let t1450 = t394 * t382;
    (t1441, t1442, t1443, t1445, t1446, t1447, t1448, t1450)
}
