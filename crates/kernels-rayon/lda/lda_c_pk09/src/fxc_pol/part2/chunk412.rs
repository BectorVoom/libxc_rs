//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 412/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk412(t2115: f64, t93: f64, t1993: f64, t2001: f64, t1732: f64, t1989: f64, t1996: f64, t1998: f64, t2003: f64, t2085: f64, t2088: f64, t2091: f64, t2104: f64, t2108: f64, t2110: f64, t2111: f64, t2114: f64, t455: f64) -> (f64, f64, f64, f64) {
    let t2116 = t93 * t2115;
    let t2121 = 0.037002892246025966_f64 * t1993;
    let t2124 = 0.14975624337724558_f64 * t2001;
    let t2126 = -t2085 * t455 / 6.0_f64 - t2088 * t455 / 6.0_f64 - t2091 * t455 / 6.0_f64 - t2104 * t455 / 6.0_f64 - t2108 + t2110 + t2111 * t455 / 6.0_f64 + t2114 * t2116 / 12.0_f64 + 0.10237773105191754_f64 * t1732 - 0.14975624337724558_f64 * t1989 - t2121 + 0.037002892246025966_f64 * t1996 - 0.037002892246025966_f64 * t1998 - t2124 + 0.14975624337724558_f64 * t2003;
    (t2116, t2121, t2124, t2126)
}
