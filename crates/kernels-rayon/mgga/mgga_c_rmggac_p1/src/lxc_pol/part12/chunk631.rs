//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 631/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk631(t7474: f64, t8443: f64, t1502: f64, t236: f64, t1971: f64, t1970: f64, t2313: f64, t5542: f64) -> (f64, f64, f64, f64) {
    let t8444 = t7474 * t8443;
    let t8446 = t236 * t1502;
    let t8447 = t1971 * t8446;
    let t8448 = t1970 * t8447;
    let t8450 = t2313 * t5542;
    (t8444, t8447, t8448, t8450)
}
