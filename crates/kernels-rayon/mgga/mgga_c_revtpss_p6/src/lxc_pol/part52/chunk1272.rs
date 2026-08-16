//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1272/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1272(t28182: f64, t8698: f64, t34261: f64, t7374: f64, t32392: f64, t7978: f64, t32394: f64, t28760: f64, t8634: f64, t34167: f64, t649: f64, t119578: f64, t125948: f64, t27123: f64, t27126: f64, t28588: f64, t28727: f64, t28935: f64, t32410: f64, t32621: f64, t4248: f64, t7732: f64, t8568: f64, t8637: f64) -> f64 {
    let t128874 = t8698 * t28182;
    let t128876 = 2.0_f64 * t34261 * t7374;
    let t128878 = 2.0_f64 * t32392 * t7978;
    let t128880 = 2.0_f64 * t32394 * t7978;
    let t128882 = 2.0_f64 * t8634 * t28760;
    let t128891 = t649 * t34167;
    let t128897 = -3.0_f64 * t119578 * t28588 - 2.0_f64 * t27123 * t8637 - 2.0_f64 * t27126 * t8637 - t28727 * t8568 + 3.0_f64 * t28935 * t8568 - 2.0_f64 * t32410 * t7732 - 2.0_f64 * t32621 * t4248 - t125948 - t128874 - t128876 - t128878 - t128880 - t128882 - t128891;
    t128897
}
