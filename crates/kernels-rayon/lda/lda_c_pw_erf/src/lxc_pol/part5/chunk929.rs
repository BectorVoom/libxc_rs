//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 929/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk929(t1185: f64, t1187: f64, t2910: f64, t2824: f64, t465: f64, t483: f64, t1131: f64, t2825: f64, t1175: f64, t1738: f64, t1179: f64, t10764: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10980 = 0.00010931146159029059_f64 * t1185 * t2910 * t1187;
    let t10983 = t2824 * t465 * t483 * t1187;
    let t10987 = 0.0006558687695417436_f64 * t2825 * t1131 * t1187;
    let t10988 = t1738 * t1175;
    let t10991 = 0.31931290694012293_f64 * t1738 * t1179;
    let t10995 = 0.0012955432484775182_f64 * t10764 * t1187;
    (t10980, t10983, t10987, t10988, t10991, t10995)
}
