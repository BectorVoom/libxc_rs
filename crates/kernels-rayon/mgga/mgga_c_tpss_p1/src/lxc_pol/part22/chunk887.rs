//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 887/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk887(t164: f64, t7886: f64, t2275: f64, t7850: f64, t2319: f64, t650: f64, t209: f64, t698: f64, t2272: f64, t713: f64, t2250: f64, t2255: f64, t2258: f64, t2267: f64, t2268: f64, t2273: f64, t2276: f64, t2314: f64, t2321: f64, t2324: f64, t2327: f64, t262: f64, t699: f64, t704: f64, t706: f64, t714: f64, t721: f64, t7814: f64, t7844: f64, t7849: f64, t7853: f64, t7858: f64, t7859: f64, t7871: f64, t7876: f64, t7879: f64, t7882: f64) -> f64 {
    let t7887 = t164 * t7886;
    let t7888 = t7850 * t2275;
    let t7895 = t650 * t2319;
    let t7899 = t209 * t698;
    let t7906 = t650 * t2272;
    let t7914 = t209 * t713;
    let t7921 = 0.35089341735807877242e1_f64 * t2327 * t7814 + 1.0_f64 * t699 * t7844 + 0.2069040516770936012e4_f64 * t7849 * t7853 - 0.10389515463408878255e3_f64 * t7858 * t7859 + 0.5848223622634646207e0_f64 * t714 * t7871 + 0.10254018858216406658e4_f64 * t7876 * t7879 + 6.0_f64 * t2273 * t7882 - 0.19298375398431042081e3_f64 * t7887 * t7888 + 0.96491876992155210402e2_f64 * t2273 * t2267 * t2275 * t704 + 0.32530743900905219526e-1_f64 * t262 * t7895 * t2321 + 0.68493333333333333332e-1_f64 * t262 * t7899 * t706 - 0.51369999999999999999e-1_f64 * t262 * t2250 * t2268 - 0.16522625736956710527e1_f64 * t262 * t7906 * t2276 + 0.10274e0_f64 * t262 * t650 * t2255 * t2258 + 0.21687162600603479684e-1_f64 * t262 * t7914 * t721 - 0.16265371950452609763e-1_f64 * t262 * t2314 * t2324;
    t7921
}
