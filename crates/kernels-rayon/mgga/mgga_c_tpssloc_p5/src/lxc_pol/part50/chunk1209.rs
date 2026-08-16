//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1209/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1209(t1058: f64, t1060: f64, t113491: f64, t113562: f64, t113576: f64, t1599: f64, t1610: f64, t1615: f64, t23633: f64, t25549: f64, t25554: f64, t30843: f64, t30889: f64, t30897: f64, t3200: f64, t32939: f64, t32943: f64, t4542: f64, t4615: f64, t4649: f64, t4684: f64, t6680: f64, t6687: f64, t6743: f64, t7619: f64, t8391: f64, t8400: f64, t8404: f64) -> f64 {
    let t119393 = t4615 * t8404 + t1610 * t30897 - t3200 * t32943 * t4684 - t113562 - 0.43864908449286038307e-1_f64 * t6680 * t32939 + t1058 * t8391 * t4649 * t1060 + t1058 * t30843 * t1615 * t1060 + 0.54831135561607547883e-2_f64 * t23633 * t113491 * t25549 - 0.14621636149762012769e-1_f64 * t113576 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t8400 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t30889 + 0.54831135561607547883e-2_f64 * t23633 * t6743 * t7619 * t25554;
    t119393
}
