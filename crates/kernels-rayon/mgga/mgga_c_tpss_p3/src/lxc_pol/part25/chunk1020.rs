//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1020/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1020(t2436: f64, t4802: f64, t10521: f64, t10520: f64, t14057: f64, t14061: f64, t14064: f64, t14065: f64, t14068: f64, t14072: f64, t14076: f64, t14080: f64, t14111: f64, t14112: f64, t1692: f64, t2439: f64, t3724: f64, t3728: f64, t750: f64, t7945: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t8112: f64, t821: f64) -> (f64, f64) {
    let t14113 = t4802 * t2436;
    let t14116 = 2.0_f64 * t10521;
    let t14117 = -6.0_f64 * t14076 * t2439 * t3728 + 3.0_f64 * t14080 * t2439 * t750 - t14113 * t1692 * t821 - 2.0_f64 * t1692 * t3724 * t3728 + t10520 - t14057 + t14061 + t14064 + t14065 + t14068 + t14072 + t14111 + t14112 + t14116 + t7945 - t7954 - t7960 + t7972 + t7975 + t8112;
    (t14116, t14117)
}
