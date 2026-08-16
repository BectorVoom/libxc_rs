//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 971/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk971(t10513: f64, t10533: f64, t10532: f64, t3380: f64, t549: f64, t1429: f64, t2365: f64, t7893: f64, t4391: f64, t3395: f64, t589: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10534 = t10533 * t10513;
    let t10536 = 0.27606906686822939767e2_f64 * t10532 * t10534;
    let t10537 = t549 * t3380;
    let t10538 = t1429 * t10537;
    let t10539 = 0.29792074959875355558e-1_f64 * t10538;
    let t10540 = t2365 * t7893;
    let t10541 = t4391 * t10540;
    let t10542 = 0.29792074959875355558e-1_f64 * t10541;
    let t10543 = t589 * t3395;
    let t10544 = t587 * t10543;
    (t10534, t10536, t10537, t10539, t10540, t10542, t10543, t10544)
}
