//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 901/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk901(t735: f64, t8017: f64, t2214: f64, t7813: f64, t7857: f64, t2332: f64, t692: f64, t2210: f64, t720: f64, t177: f64, t2240: f64, t737: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8019 = 0.5848223622634646207e0_f64 * t735 * t8017;
    let t8021 = t7857 * t7813 * t2214;
    let t8023 = 0.10389515463408878255e3_f64 * t735 * t8021;
    let t8024 = t692 * t2332;
    let t8027 = t2210 * t7813 * t720;
    let t8029 = 0.35089341735807877242e1_f64 * t735 * t8027;
    let t8034 = t2240 * t177;
    let t8035 = t8034 * t737;
    (t8019, t8021, t8023, t8024, t8027, t8029, t8035)
}
