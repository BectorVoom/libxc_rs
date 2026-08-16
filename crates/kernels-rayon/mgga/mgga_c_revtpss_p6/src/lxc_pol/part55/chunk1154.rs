//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1154/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1154(t2061: f64, t7063: f64, t25410: f64, t25413: f64, t120111: f64, t120114: f64, t120117: f64, t120132: f64, t119823: f64, t121817: f64, t121913: f64, t32474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122002 = t7063 * t2061;
    let t122003 = t122002 * t25410;
    let t122004 = t122003 * t25413;
    let t122008 = 0.7437465841810202164e-5_f64 * t120111;
    let t122009 = 0.39671442800215618342e-4_f64 * t120114;
    let t122010 = 0.47023883532522246276e-4_f64 * t120117;
    let t122015 = 0.26773803678175077507e-4_f64 * t120132;
    let t122024 = t119823 * t121817;
    let t122026 = t32474 * t121913;
    (t122002, t122003, t122004, t122008, t122009, t122010, t122015, t122024, t122026)
}
