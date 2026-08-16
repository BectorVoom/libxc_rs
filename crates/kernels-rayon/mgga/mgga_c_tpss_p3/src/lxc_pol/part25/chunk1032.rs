//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1032/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1032(t10520: f64, t14061: f64, t14064: f64, t14065: f64, t14068: f64, t14072: f64, t14111: f64, t14112: f64, t14116: f64, t7945: f64, t7954: f64, t7960: f64, t7972: f64, t7975: f64, t8112: f64, t8117: f64) -> f64 {
    let t14262 = t14061 + t7945 + t14064 + t14065 + t14068 + t14072 - t7954 - t7960 + t7972 + t7975 + t14111 + t14112 + t10520 + t14116 + t8112 - t8117;
    t14262
}
