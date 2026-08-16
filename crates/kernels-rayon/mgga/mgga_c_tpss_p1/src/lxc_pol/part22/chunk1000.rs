//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1000/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1000(t10754: f64, t750: f64, t2133: f64, t3657: f64, t10552: f64, t778: f64, t10735: f64, t10745: f64, t10751: f64, t1373: f64, t1375: f64, t222: f64, t224: f64, t2353: f64, t2358: f64, t2361: f64, t3650: f64, t3656: f64, t3658: f64, t3661: f64, t776: f64, t779: f64) -> f64 {
    let t10755 = t10754 * t750;
    let t10758 = t3657 * t2133;
    let t10761 = t778 * t10552;
    let t10764 = -t10735 * t224 - 24.0_f64 * t10745 * t3658 + 60.0_f64 * t10751 * t3656 - 24.0_f64 * t10755 * t3656 - 12.0_f64 * t10758 * t3656 + 3.0_f64 * t10761 * t222 - 12.0_f64 * t1373 * t2358 + 3.0_f64 * t1373 * t2361 + 3.0_f64 * t1375 * t2353 + 6.0_f64 * t3650 * t779 + 6.0_f64 * t3661 * t776;
    t10764
}
