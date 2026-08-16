//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1065/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1065(t11648: f64, t2741: f64, t1407: f64, t2725: f64, t2682: f64, t3941: f64, t11631: f64, t11637: f64, t11641: f64, t11647: f64, t1467: f64, t2685: f64, t2722: f64, t2740: f64, t3928: f64, t3956: f64, t8509: f64, t8514: f64, t8958: f64, t8976: f64, t9042: f64) -> f64 {
    let t11649 = t2741 * t11648;
    let t11652 = t1407 * t2725;
    let t11653 = t2741 * t11652;
    let t11659 = t2682 * t3941 / 432.0_f64;
    let t11660 = -t8509 * t11631 / 4608.0_f64 + t8976 * t3956 / 288.0_f64 + t2722 * t11637 / 768.0_f64 - t11641 / 1296.0_f64 - t2685 * t3928 / 54.0_f64 + t11647 + t9042 - t2740 * t11649 / 2304.0_f64 + t8514 * t11653 / 2304.0_f64 + 19.0_f64 / 1728.0_f64 * t8958 * t1467 - t11659;
    t11660
}
