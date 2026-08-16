//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 863/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk863(t5818: f64, t5821: f64, t5930: f64, t5932: f64, t5934: f64, t5936: f64, t5940: f64, t5945: f64, t5950: f64, t5952: f64, t5955: f64, t5959: f64) -> f64 {
    let t7842 = -0.571528e-1_f64 * t5930 + 4.0_f64 * t5932 + 4.0_f64 * t5934 - t5818 + t5821 + 0.1445810840040231979e-1_f64 * t5936 + t5940 + t5945 - t5950 - 0.20010214504933333333e-2_f64 * t5952 - 0.40020429009866666666e-2_f64 * t5955 + t5959;
    t7842
}
