//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 873/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk873<F: Float>(t39056: F, t7844: F, t39876: F, t40850: F, t40852: F, t40854: F, t40856: F, t40858: F, t40860: F, t40862: F, t40866: F, t40868: F, t40870: F, t39060: F, t7785: F, t39880: F) -> (F, F, F) {
    let t40872 = t7844 * t39056;
    let t40874 = t7844 * t39876;
    let t40876 = -0.13637330827122670864e0 * t40850 - 0.6818665413561335432e-1 * t40852 - 0.27274661654245341728e-1 * t40854 - 0.13637330827122670864e-1 * t40856 + 0.20455996240684006296e-1 * t40858 + 0.10227998120342003148e-1 * t40860 - 0.27274661654245341728e-1 * t40862 - 0.13637330827122670864e-1 * t40866 - 0.13637330827122670864e-1 * t40868 - 0.68186654135613354322e-2 * t40870 - 0.40911992481368012592e-1 * t40872 - 0.20455996240684006296e-1 * t40874;
    let t40877 = t7785 * t39060;
    let t40879 = t7785 * t39880;
    (t40876, t40877, t40879)
}
