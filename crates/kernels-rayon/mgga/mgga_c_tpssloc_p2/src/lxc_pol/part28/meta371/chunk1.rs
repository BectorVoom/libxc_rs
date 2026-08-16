//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1410/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1410(t2906: f64, t4475: f64, t2932: f64, t4471: f64, t950: f64, t1581: f64, t1569: f64, t2862: f64, t10747: f64, t10771: f64, t10811: f64, t10825: f64, t10828: f64, t14429: f64, t14432: f64, t14436: f64, t14439: f64, t14443: f64, t14450: f64, t14453: f64, t2861: f64, t2886: f64, t2905: f64, t2930: f64, t4454: f64, t4476: f64) -> f64 {
    let t14456 = t4475 * t2906;
    let t14459 = t4471 * t2932;
    let t14460 = t14459 * t950;
    let t14463 = t1581 * t2906;
    let t14466 = t1569 * t2862;
    let t14469 = -2.0_f64 * t2861 * t14429 - 0.19298375398431042081e3_f64 * t10771 * t14432 + 0.64327917994770140268e2_f64 * t2886 * t14436 + 0.32163958997385070134e2_f64 * t2886 * t14439 + 0.2069040516770936012e4_f64 * t10811 * t14443 - 0.23392894490538584828e1_f64 * t10747 * t4454 + 0.34631718211362927518e2_f64 * t10825 * t4476 - 0.23392894490538584828e1_f64 * t2905 * t14450 - 0.11696447245269292414e1_f64 * t2905 * t14453 - 0.10389515463408878255e3_f64 * t10828 * t14456 + 0.34631718211362927518e2_f64 * t2930 * t14460 + 0.35089341735807877242e1_f64 * t2930 * t14463 + 6.0_f64 * t2886 * t14466;
    t14469
}
