//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1361/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1361(t24807: f64, t24826: f64, t225: f64, t24705: f64, t24574: f64, t24860: f64, t24594: f64, t24847: f64, t974: f64, t27551: f64, t7327: f64, t11154: f64, t1201: f64, t1215: f64, t1244: f64, t1246: f64, t24589: f64, t24757: f64, t24776: f64, t24792: f64, t24804: f64, t24812: f64, t24858: f64, t27489: f64, t3252: f64, t3565: f64, t3604: f64, t4978: f64, t7283: f64, t7362: f64, t7363: f64, t7364: f64, t7389: f64, t85827: f64, t86016: f64) -> f64 {
    let t86057 = t24826 * t24807;
    let t86059 = t24705 * t225;
    let t86073 = t24574 * t24860;
    let t86076 = t24847 * t974 * t24594;
    let t86077 = t7327 * t27551;
    let t86089 = 0.49348022005446793095e-1_f64 * t24812 * t27489 * t85827 * t4978 + 0.82246703342411321826e-2_f64 * t86057 + 0.82246703342411321826e-2_f64 * t24589 * t86059 * t7364 + 3.0_f64 * t1201 * t24792 + 3.0_f64 * t1244 * t24757 * t1215 * t1246 - 0.82246703342411321826e-2_f64 * t7283 * t7362 * t24858 * t3252 - 0.54831135561607547883e-2_f64 * t86073 + 0.10966227112321509577e-1_f64 * t86076 * t86077 * t86016 + 0.21932454224643019154e-1_f64 * t7283 * t24776 * t7363 * t11154 + 3.0_f64 * t3565 * t7389 + 3.0_f64 * t3604 * t24804;
    t86089
}
