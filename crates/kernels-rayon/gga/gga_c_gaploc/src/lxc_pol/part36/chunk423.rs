//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 423/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk423(t2684: f64, t3500: f64, t3009: f64, t935: f64, t1445: f64, t2087: f64, t3431: f64, t836: f64, t568: f64, t317: f64, t3275: f64, t3283: f64, t3298: f64, t3313: f64, t3463: f64, t3465: f64, t3469: f64, t3472: f64, t3476: f64, t3479: f64, t3480: f64, t3486: f64, t3491: f64, t3494: f64, t3496: f64, t3499: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t3501 = t2684 * t3500;
    let t3502 = 0.19171462976960374838e0_f64 * t3501;
    let t3503 = t3009 * t935;
    let t3504 = t1445 * t3503;
    let t3506 = 0.69017266717057349418e1_f64 * t2087 * t3504;
    let t3507 = t836 * t3431;
    let t3508 = t568 * t3507;
    let t3511 = t3463 + 0.35750489951850426669e0_f64 * t3465 * t317 + t3469 - t3472 + t3275 - t3283 - t3476 + t3479 - 0.35750489951850426669e0_f64 * t797 * t3480 - t3486 - t3491 + t3494 - 0.23005755572352449806e1_f64 * t813 * t3496 - t3298 - t3499 + t3313 + t3502 - t3506 + 0.23005755572352449806e1_f64 * t833 * t3508;
    (t3503, t3504, t3507, t3508, t3511)
}
