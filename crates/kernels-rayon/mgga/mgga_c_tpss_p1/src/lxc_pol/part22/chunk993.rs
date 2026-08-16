//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 993/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk993(t10509: f64, t10512: f64, t10518: f64, t10520: f64, t10521: f64, t10522: f64, t10523: f64, t10524: f64, t10526: f64, t10528: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t8112: f64, t8117: f64) -> f64 {
    let t10682 = t10509 + t10512 - t7954 - t7960 + t7972 + t7975 + t10518 + t10520 + t10521 + t10522 + t8112 - t8117 + t10523 - t10524 + t10526 + t10528;
    t10682
}
