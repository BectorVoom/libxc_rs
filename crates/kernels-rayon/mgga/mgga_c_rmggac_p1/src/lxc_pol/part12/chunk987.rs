//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 987/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk987(t262: f64, t40864: f64, t7782: f64, t40488: f64, t7835: f64, t39373: f64, t39056: f64, t7844: f64, t39876: f64, t40850: f64, t40852: f64, t40854: f64, t40856: f64, t40858: f64, t40860: f64, t40862: f64) -> (f64, f64) {
    let t40865 = t262 * t40864;
    let t40866 = t7782 * t40865;
    let t40868 = t7835 * t40488;
    let t40870 = t7835 * t39373;
    let t40872 = t7844 * t39056;
    let t40874 = t7844 * t39876;
    let t40876 = -0.13637330827122670864e0_f64 * t40850 - 0.6818665413561335432e-1_f64 * t40852 - 0.27274661654245341728e-1_f64 * t40854 - 0.13637330827122670864e-1_f64 * t40856 + 0.20455996240684006296e-1_f64 * t40858 + 0.10227998120342003148e-1_f64 * t40860 - 0.27274661654245341728e-1_f64 * t40862 - 0.13637330827122670864e-1_f64 * t40866 - 0.13637330827122670864e-1_f64 * t40868 - 0.68186654135613354322e-2_f64 * t40870 - 0.40911992481368012592e-1_f64 * t40872 - 0.20455996240684006296e-1_f64 * t40874;
    (t40865, t40876)
}
