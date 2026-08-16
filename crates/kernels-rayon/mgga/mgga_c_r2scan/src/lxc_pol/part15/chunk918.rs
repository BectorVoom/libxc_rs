//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 918/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk918(t1103: f64, t1783: f64, t1053: f64, t1102: f64, t357: f64, t862: f64, t255: f64, t868: f64, t258: f64) -> (f64, f64, f64, f64, f64) {
    let t10641 = t1103 * t1783;
    let t10643 = t1102 * t1053 * t10641;
    let t10645 = t862 * t357;
    let t10646 = t868 * t255;
    let t10647 = t10646 * t258;
    (t10641, t10643, t10645, t10646, t10647)
}
