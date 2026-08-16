//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1193/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1193(t12916: f64, t34944: f64, t8946: f64, t104527: f64, t3736: f64, t8937: f64, t1032: f64, t1203: f64, t1214: f64, t124706: f64, t124744: f64, t124887: f64, t125048: f64, t125059: f64, t131631: f64, t2150: f64, t33436: f64, t33441: f64, t33461: f64, t33462: f64, t33477: f64, t33478: f64, t33487: f64, t34908: f64, t34931: f64, t34939: f64, t3555: f64, t473: f64, t5215: f64, t5245: f64, t5407: f64, t5497: f64, t7652: f64, t8925: f64, t8931: f64) -> f64 {
    let t132018 = t8946 * t34944 * t12916;
    let t132032 = t8937 * t104527 * t3736;
    let t132047 = 0.56468933516960933998e-3_f64 * t3555 * t1032 * t8925 * t34931 - 0.17135921299530705785e1_f64 * t33477 * t33478 * t8931 * t5497 + 0.34694512752820797848e1_f64 * t124887 * t7652 * t131631 + 0.66110807482757352569e-3_f64 * t132018 - 0.37645955677973955998e-3_f64 * t125059 + 0.34271842599061411569e1_f64 * t33461 * t33462 * t34908 * t1203 - 0.17347256376410398924e1_f64 * t33436 * t2150 * t473 * t5215 + 0.24791552806034007214e-3_f64 * t124744 * t5407 + 0.11423947533020470523e1_f64 * t132032 * t33487 + 0.17347256376410398924e1_f64 * t33441 * t2150 * t473 * t5245 - 0.51407763898592117355e1_f64 * t124706 * t33462 * t34939 * t1203 + 0.6854368519812282314e1_f64 * t125048 * t33462 * t34939 * t1214;
    t132047
}
