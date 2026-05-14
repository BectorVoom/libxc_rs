//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 715/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk715<F: Float>(t4678: F, t62: F, t1260: F, t70: F, t4630: F, t1273: F, t1276: F, t155: F, t174: F, t1215: F, t1319: F, t331: F, t449: F, t388: F, t405: F, t1268: F, t1286: F) -> (F, F, F, F, F, F, F, F) {
    let t4679 = t62 * t4678;
    let t4681 = 1.0 / t1260 / t70;
    let t4682 = t4630 * t4681;
    let t4687 = t174 * t155 * t1273 * t1276;
    let t4688 = 0.10685e0 * t4687;
    let t4689 = t155 * t1215;
    let t4697 = t155 * t1319;
    let t4701 = t331 * t449;
    let t4708 = t331 * t388;
    let t4710 = t174 * t4708 * t405;
    let t4711 = 0.71233333333333333334e-1 * t4710;
    let t4713 = t174 * t1268 * t1286;
    (t4679, t4682, t4688, t4689, t4697, t4701, t4711, t4713)
}
