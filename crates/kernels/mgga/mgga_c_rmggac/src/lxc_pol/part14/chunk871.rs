//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 871/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk871<F: Float>(t118: F, t27101: F, t305: F, t326: F, t38384: F, t38787: F, t39573: F, t40616: F, t40824: F, t40827: F, t40832: F, t40834: F, t40842: F, t40844: F, t40846: F, t4669: F, t794: F, t833: F, t8936: F, t8946: F) -> (F,) {
    let t40848 = -0.17961362552795712846e0 * t4669 * t8946 * t833 - 0.23948483403727617128e0 * t27101 * t8936 * t794 - 0.17961362552795712846e0 * t40824 - 0.5987120850931904282e-1 * t40827 - 0.59871208509319042821e-1 * t326 * t38384 + t40832 - 0.17961362552795712846e0 * t40834 - 0.11974241701863808564e0 * t326 * t38787 + 0.11974241701863808564e0 * t305 * t39573 - 0.39914139006212695214e-1 * t118 * t40616 + 0.2993560425465952141e-1 * t40842 + 0.81823984962736025184e-1 * t40844 + 0.40911992481368012592e-1 * t40846;
    (t40848,)
}
