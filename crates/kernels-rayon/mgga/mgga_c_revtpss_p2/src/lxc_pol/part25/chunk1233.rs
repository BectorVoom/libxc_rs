//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1233/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1233(t92975: f64, t92942: f64, t92944: f64, t92946: f64, t92948: f64, t92952: f64, t92956: f64, t92958: f64, t92960: f64, t92963: f64, t92966: f64, t92969: f64, t92971: f64, t92973: f64) -> f64 {
    let t92976 = 0.91476005056713590805e-4_f64 * t92975;
    let t92977 = 0.25724410870841842183e-2_f64 * t92942 - 0.25724410870841842184e-1_f64 * t92944 + 0.51448821741683684367e-2_f64 * t92946 + 0.51448821741683684367e-2_f64 * t92948 - 0.48018900292238105409e-1_f64 * t92952 + 0.6098400337114239387e-3_f64 * t92956 + 0.51448821741683684367e-2_f64 * t92958 - 0.12862205435420921092e-2_f64 * t92960 + 0.15246000842785598468e-4_f64 * t92963 - 0.1084295579938911763e-3_f64 * t92966 - 35.0_f64 / 72.0_f64 * t92969 + 7.0_f64 / 48.0_f64 * t92971 - t92973 / 48.0_f64 + t92976;
    t92977
}
