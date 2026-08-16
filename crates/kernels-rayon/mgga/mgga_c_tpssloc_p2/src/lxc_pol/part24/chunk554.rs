//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 554/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk554(t2932: f64, t2951: f64, t959: f64, t2262: f64, t338: f64, t964: f64, t969: f64, t615: f64, t972: f64) -> (f64, f64, f64, f64, f64) {
    let t2952 = t2951 * t2932;
    let t2954 = 0.17315859105681463759e2_f64 * t959 * t2952;
    let t2955 = t2262 * t338;
    let t2958 = t964 * t969;
    let t2960 = t615 * t972;
    (t2952, t2954, t2955, t2958, t2960)
}
