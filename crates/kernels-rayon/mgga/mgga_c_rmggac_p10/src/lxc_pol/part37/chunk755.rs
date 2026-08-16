//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 755/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk755(t446: f64, t511: f64, t558: f64, t14117: f64, t68448: f64, t68455: f64, t9205: f64, t14022: f64, t14027: f64, t15339: f64, t458: f64, t1430: f64, t236: f64) -> (f64, f64, f64, f64, f64) {
    let t73699 = t511 * t558 * t446;
    let t73701 = t68448 * t14117 * t73699;
    let t73704 = t68455 * t14117 * t9205;
    let t73708 = t15339 * t458 * t14022 * t14027;
    let t73712 = t236 * t1430;
    (t73699, t73701, t73704, t73708, t73712)
}
