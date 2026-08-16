//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1416/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1416(t34530: f64, t34533: f64, t34547: f64, t36969: f64, t36970: f64, t36971: f64, t36972: f64, t36973: f64, t36974: f64, t36977: f64, t36978: f64, t34550: f64, t34567: f64, t36982: f64, t36983: f64, t36984: f64, t36985: f64, t36986: f64, t36987: f64, t36989: f64, t36990: f64, t36991: f64) -> (f64, f64) {
    let t38583 = t36969 + t36970 + t36971 + t36972 - t36973 - t36974 + 0.56912804804009946682e-7_f64 * t34530 + 0.68761854623411138864e-8_f64 * t34533 - t36977 + t36978 + 0.56399158975894962978e-8_f64 * t34547;
    let t38586 = 0.90579542097823505428e-7_f64 * t34550 + t36982 + t36983 + t36984 + t36985 - t36986 - t36987 - 0.6629778687778673199e-7_f64 * t34567 + t36989 - t36990 - t36991;
    (t38583, t38586)
}
