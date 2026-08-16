//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 892/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk892(t34785: f64, t34788: f64, t34794: f64, t34799: f64, t38819: f64, t38823: f64, t38826: f64, t38833: f64, t38838: f64, t38841: f64, t38846: f64, t38850: f64, t38854: f64, t38858: f64, t38861: f64, t38864: f64, t5928: f64, t8390: f64) -> f64 {
    let t44972 = -0.23948483403727617128e0_f64 * t5928 * t8390 + t38819 - t38823 + 0.60975299583150056628e-3_f64 * t38826 - t34785 + t34788 - t34794 - 0.72042316457491791906e-3_f64 * t34799 + 0.60975299583150056628e-3_f64 * t38833 + t38838 - 0.86737941314158990623e-4_f64 * t38841 - 0.86737941314158990623e-4_f64 * t38846 - 0.14408463291498358381e-2_f64 * t38850 - t38854 + t38858 + t38861 + t38864;
    t44972
}
