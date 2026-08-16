//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 704/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk704(t3139: f64, t465: f64, t7472: f64, t1986: f64, t305: f64, t7476: f64, t118: f64, t2001: f64, t498: f64, t665: f64, t2000: f64, t797: f64) -> (f64, f64, f64, f64, f64) {
    let t69618 = t465 * t3139;
    let t69619 = t7472 * t69618;
    let t69621 = t1986 * t305 * t7476;
    let t69626 = t2001 * t118 * t665 * t498;
    let t69629 = t2000 * t797;
    (t69618, t69619, t69621, t69626, t69629)
}
