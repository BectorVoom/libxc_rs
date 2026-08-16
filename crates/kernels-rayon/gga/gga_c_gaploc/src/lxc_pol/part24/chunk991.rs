//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 991/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk991(t10740: f64, t8528: f64, t883: f64, t2562: f64, t943: f64, t2549: f64, t3437: f64, t10716: f64, t10720: f64, t10722: f64, t10733: f64, t10735: f64, t10739: f64, t1897: f64, t3434: f64, t3452: f64, t650: f64, t681: f64, t9754: f64, t9762: f64) -> (f64, f64) {
    let t10741 = 0.32043859292259267849e-3_f64 * t10740;
    let t10742 = t883 * t8528;
    let t10743 = t2562 * t10742;
    let t10744 = t943 * t10743;
    let t10745 = 0.32043859292259267849e-3_f64 * t10744;
    let t10746 = t2549 * t3437;
    let t10747 = 0.32043859292259267849e-3_f64 * t10746;
    let t10748 = t10716 + t10720 + 0.76905262301422242837e-2_f64 * t1897 * t10722 - 0.76905262301422242837e-2_f64 * t681 * t3452 + 0.10254034973522965712e-1_f64 * t650 * t3434 - 0.10254034973522965712e-1_f64 * t650 * t3452 + t9754 + t9762 + t10733 + t10735 - t10739 - t10741 - t10745 + t10747;
    (t10743, t10748)
}
