//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 947/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk947(t10709: f64, t959: f64, t2904: f64, t2925: f64, t950: f64, t2880: f64, t2888: f64, t931: f64, t2924: f64, t952: f64, t2932: f64, t2836: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10711 = 0.35089341735807877242e1_f64 * t959 * t10709;
    let t10713 = t2904 * t950 * t2925;
    let t10715 = 0.35089341735807877242e1_f64 * t959 * t10713;
    let t10717 = t2880 * t2888 * t931;
    let t10720 = t952 * t2924;
    let t10723 = t2924 * t2932;
    let t10724 = t10723 * t950;
    let t10727 = t914 * t2836;
    (t10711, t10715, t10717, t10720, t10724, t10727)
}
