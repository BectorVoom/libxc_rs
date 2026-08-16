//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2011/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011(t23471: f64, t23482: f64, t10889: f64, t23535: f64, t3033: f64, t1016: f64, t3034: f64, t1930: f64, t23418: f64, t3180: f64, t10401: f64, t23417: f64) -> (f64, f64, f64, f64, f64) {
    let t82943 = t23482 * t23471;
    let t82956 = t3033 * t23535 * t10889;
    let t82985 = 1.0_f64 / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t83008 = t3180 * t23418;
    let t83015 = t23417 * t10401;
    (t82943, t82956, t82986, t83008, t83015)
}
