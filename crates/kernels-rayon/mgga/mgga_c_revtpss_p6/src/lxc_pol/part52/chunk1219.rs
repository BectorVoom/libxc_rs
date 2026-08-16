//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1219/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1219(t102854: f64, t121716: f64, t121793: f64, t127582: f64, t127892: f64, t1583: f64, t18875: f64, t1940: f64, t1962: f64, t198: f64, t207: f64, t2403: f64, t25445: f64, t26425: f64, t26585: f64, t27363: f64, t27384: f64, t28460: f64, t28472: f64, t32491: f64, t34080: f64, t4343: f64, t4537: f64, t7086: f64, t7432: f64, t775: f64, t7782: f64, t8657: f64, t890: f64, t892: f64, t92742: f64) -> f64 {
    let t128014 = t198 * t207 * t127892 * t892 + 6.0_f64 * t26425 * t25445 * t18875 - t1940 * t26585 * t7782 - t1940 * t32491 * t4537 + 3.0_f64 * t2403 * t34080 * t775 + 2.0_f64 * t1940 * t121793 * t27384 - t1940 * t121716 * t1583 - t1940 * t28460 * t7086 - t1940 * t102854 * t1962 + 3.0_f64 * t2403 * t8657 * t4343 - t1940 * t7432 * t27363 - 6.0_f64 * t28472 * t92742 * t27384 - t1940 * t127582 * t890 - 3.0_f64 * t2403 * t32491 * t18875;
    t128014
}
