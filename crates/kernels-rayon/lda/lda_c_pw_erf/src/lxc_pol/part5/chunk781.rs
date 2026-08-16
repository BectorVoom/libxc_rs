//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 781/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk781(t6232: f64, t6235: f64, t6238: f64, t6241: f64, t6246: f64, t6248: f64, t6250: f64, t6252: f64, t6254: f64, t6258: f64, t6260: f64, t6267: f64, t6269: f64, t6274: f64, t6279: f64, t6284: f64) -> f64 {
    let t7239 = t6232 - t6235 - t6238 + t6241 + t6246 + t6248 - t6250 - t6252 + t6254 + t6258 + t6260 + t6267 - t6269 + t6274 + t6279 - t6284;
    t7239
}
