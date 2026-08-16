//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 414/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk414(t532: f64, t535: f64, t159: f64, t285: f64, t545: f64, t762: f64, t147: f64, t39: f64, t169: f64, t274: f64, t301: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1457 = t532 * t535;
    let t1459 = t1457 * t159 * t285;
    let t1463 = 0.58113483035773838734e-3_f64 * t762 * t545 * t285;
    let t1464 = t39 * t147;
    let t1467 = 0.13559812708347229038e-2_f64 * t1464 * t159 * t285;
    let t1471 = 0.19816831758676854261e0_f64 * t169 * t366 * t274 * t301;
    (t1457, t1459, t1463, t1464, t1467, t1471)
}
