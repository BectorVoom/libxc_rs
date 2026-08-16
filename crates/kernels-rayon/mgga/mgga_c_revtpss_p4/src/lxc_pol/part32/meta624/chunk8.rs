//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1976/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1976(t102293: f64, t102296: f64, t102298: f64, t102306: f64, t102309: f64, t102316: f64, t108178: f64, t25930: f64, t26304: f64, t28008: f64, t28915: f64, t8104: f64, t96280: f64, t96284: f64, t96287: f64, t96289: f64, t96298: f64, t97933: f64) -> f64 {
    let t109533 = -0.34270468708064099208e-2_f64 * t96280 - 0.8673628188205199462e0_f64 * t28008 * t8104 - 0.68540937416128198416e-1_f64 * t102293 - 0.19274729307122665472e-1_f64 * t102296 - 0.17347256376410398924e1_f64 * t25930 * t26304 * t108178 - t96284 + 0.34270468708064099208e-1_f64 * t102298 + t102306 - 0.17347256376410398924e1_f64 * t97933 * t28915 - 0.22849835011101738147e-2_f64 * t96287 + t102309 + 0.17135234354032049604e-1_f64 * t96289 + 0.19274729307122665472e-1_f64 * t102316 + 0.96373646535613327357e-2_f64 * t96298;
    t109533
}
