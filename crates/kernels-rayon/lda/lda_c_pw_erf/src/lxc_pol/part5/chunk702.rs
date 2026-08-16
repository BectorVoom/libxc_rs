//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 702/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk702(t1318: f64, t6256: f64, t2007: f64, t4738: f64, t6217: f64, t6219: f64, t6222: f64, t6224: f64, t6225: f64, t6228: f64, t6232: f64, t6235: f64, t6238: f64, t6241: f64, t6246: f64, t6248: f64, t6250: f64, t6252: f64, t6254: f64) -> (f64, f64, f64) {
    let t6258 = 8.0_f64 / 45.0_f64 * t1318 * t6256;
    let t6260 = 16.0_f64 / 45.0_f64 * t4738 * t2007;
    let t6261 = t6217 + t6219 + t6222 - t6224 + 2.0_f64 / 9.0_f64 * t6225 + t6228 + t6232 - t6235 - t6238 + t6241 + t6246 + t6248 - t6250 - t6252 + t6254 + t6258 + t6260;
    (t6258, t6260, t6261)
}
