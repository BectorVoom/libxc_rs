//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 696/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk696(t5: f64, t518: f64, t586: f64, t4849: f64, t4850: f64, t4851: f64, t4852: f64, t4853: f64, t5309: f64, t5312: f64) -> (f64, f64, f64) {
    let t5314 = t5 * t518;
    let t5315 = t586 * t5314;
    let t5317 = -0.17261666666666666667e1_f64 * t5309 + 0.11507777777777777778e1_f64 * t5312 - 0.53702962962962962964e1_f64 * t5315 - t4849 + t4850 - t4851 - t4852 - t4853;
    (t5314, t5315, t5317)
}
