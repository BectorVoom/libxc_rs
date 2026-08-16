//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1789/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1789(t1544: f64, t1583: f64, t18875: f64, t1940: f64, t198: f64, t207: f64, t2071: f64, t2403: f64, t26585: f64, t26590: f64, t27375: f64, t27384: f64, t28455: f64, t28460: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7428: f64, t7432: f64, t775: f64, t8020: f64, t890: f64, t892: f64) -> f64 {
    let t28522 = t198 * t207 * t28455 * t892 + 3.0_f64 * t1544 * t2403 * t7428 - t1583 * t1940 * t26585 - 3.0_f64 * t18875 * t2403 * t7432 + 2.0_f64 * t1940 * t26590 * t27384 - t1940 * t28460 * t890 - t1940 * t4537 * t7432 + 3.0_f64 * t2071 * t2403 * t4343 + 6.0_f64 * t2071 * t4433 * t4541 - 3.0_f64 * t2403 * t27375 * t7432 + 3.0_f64 * t2403 * t775 * t8020;
    t28522
}
