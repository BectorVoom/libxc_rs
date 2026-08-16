//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 902/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk902(t7813: f64, t7875: f64, t7878: f64, t735: f64, t1985: f64, t725: f64, t2337: f64, t2428: f64, t823: f64, t200: f64, t45: f64, t202: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8038 = t7875 * t7813 * t7878;
    let t8040 = 0.10254018858216406658e4_f64 * t735 * t8038;
    let t8042 = t725 * t1985;
    let t8043 = t2337 * t8042;
    let t8045 = t2428 * t823;
    let t8050 = 1.0_f64 / t200 / t45;
    let t8061 = 1.0_f64 / t202 / t57;
    (t8038, t8040, t8043, t8045, t8050, t8061)
}
