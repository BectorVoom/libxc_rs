//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 966/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk966(t14953: f64, t14955: f64, t14957: f64, t14958: f64, t14962: f64, t14965: f64, t14968: f64, t14971: f64, t14974: f64, t14977: f64, t14980: f64, t14983: f64, t14986: f64, t14989: f64, t14992: f64, t14995: f64, t14999: f64, t15000: f64, t15004: f64, t15007: f64, t3051: f64, t3139: f64, t462: f64, t92: f64) -> f64 {
    let t15010 = -t14953 - t14955 + t14957 - 2.0_f64 / 9.0_f64 * t462 * t14958 - 10.0_f64 / 27.0_f64 * t462 * t14962 + 8.0_f64 / 9.0_f64 * t3139 * t14965 + t462 * t14968 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t462 * t14971 - 2.0_f64 / 3.0_f64 * t462 * t14974 - 2.0_f64 * t462 * t14977 - 2.0_f64 / 3.0_f64 * t462 * t14980 - 4.0_f64 / 3.0_f64 * t3139 * t14983 + 2.0_f64 / 3.0_f64 * t462 * t14986 - 8.0_f64 / 3.0_f64 * t3139 * t14989 + t462 * t14992 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t462 * t14995 - t14999 + 2.0_f64 / 3.0_f64 * t462 * t15000 - t92 * t15004 + 2.0_f64 / 3.0_f64 * t3051 * t15007;
    t15010
}
