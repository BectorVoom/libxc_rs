//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 985/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk985(t38746: f64, t7785: f64, t39689: f64, t118: f64, t27101: f64, t305: f64, t326: f64, t38384: f64, t38787: f64, t39573: f64, t40616: f64, t40824: f64, t40827: f64, t40832: f64, t40834: f64, t40842: f64, t4669: f64, t794: f64, t833: f64, t8936: f64, t8946: f64) -> f64 {
    let t40844 = t7785 * t38746;
    let t40846 = t7785 * t39689;
    let t40848 = -0.17961362552795712846e0_f64 * t4669 * t8946 * t833 - 0.23948483403727617128e0_f64 * t27101 * t8936 * t794 - 0.17961362552795712846e0_f64 * t40824 - 0.5987120850931904282e-1_f64 * t40827 - 0.59871208509319042821e-1_f64 * t326 * t38384 + t40832 - 0.17961362552795712846e0_f64 * t40834 - 0.11974241701863808564e0_f64 * t326 * t38787 + 0.11974241701863808564e0_f64 * t305 * t39573 - 0.39914139006212695214e-1_f64 * t118 * t40616 + 0.2993560425465952141e-1_f64 * t40842 + 0.81823984962736025184e-1_f64 * t40844 + 0.40911992481368012592e-1_f64 * t40846;
    t40848
}
