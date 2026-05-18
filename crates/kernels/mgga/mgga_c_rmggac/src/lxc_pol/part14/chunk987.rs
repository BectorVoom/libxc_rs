//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 987/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk987<F: Float>(t262: F, t40864: F, t7782: F, t40488: F, t7835: F, t39373: F, t39056: F, t7844: F, t39876: F, t40850: F, t40852: F, t40854: F, t40856: F, t40858: F, t40860: F, t40862: F) -> (F, F) {
    let t40865 = t262 * t40864;
    let t40866 = t7782 * t40865;
    let t40868 = t7835 * t40488;
    let t40870 = t7835 * t39373;
    let t40872 = t7844 * t39056;
    let t40874 = t7844 * t39876;
    let t40876 = -F::new(0.13637330827122670864e0) * t40850 - F::new(0.6818665413561335432e-1) * t40852 - F::new(0.27274661654245341728e-1) * t40854 - F::new(0.13637330827122670864e-1) * t40856 + F::new(0.20455996240684006296e-1) * t40858 + F::new(0.10227998120342003148e-1) * t40860 - F::new(0.27274661654245341728e-1) * t40862 - F::new(0.13637330827122670864e-1) * t40866 - F::new(0.13637330827122670864e-1) * t40868 - F::new(0.68186654135613354322e-2) * t40870 - F::new(0.40911992481368012592e-1) * t40872 - F::new(0.20455996240684006296e-1) * t40874;
    (t40865, t40876)
}
