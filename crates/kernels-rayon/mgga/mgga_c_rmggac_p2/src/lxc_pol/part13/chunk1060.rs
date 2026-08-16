//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1060/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1060(t39698: f64, t39701: f64, t35285: f64, t35327: f64, t35337: f64, t39706: f64, t39709: f64, t39711: f64, t39715: f64, t39717: f64, t39721: f64, t39726: f64, t39731: f64, t39733: f64, t39735: f64, t39742: f64, t39748: f64) -> f64 {
    let t43107 = 0.10909864661698136692e0_f64 * t39698;
    let t43108 = 0.47896966807455234256e0_f64 * t39701;
    let t43124 = t43107 - t43108 - 0.40911992481368012596e-1_f64 * t39706 + 0.11918087970123395032e-3_f64 * t35285 + 0.1702583995731913576e-4_f64 * t39709 - 0.2553875993597870364e-4_f64 * t39711 + 0.2553875993597870364e-4_f64 * t39715 - 0.2553875993597870364e-4_f64 * t39717 - 0.5107751987195740728e-4_f64 * t39721 + 0.2553875993597870364e-4_f64 * t39726 + 0.85129199786595678799e-5_f64 * t39731 - 0.85129199786595678799e-5_f64 * t39733 + 0.1702583995731913576e-4_f64 * t39735 - 0.2553875993597870364e-3_f64 * t39742 - 0.13242319966803772257e-3_f64 * t35327 - 0.11918087970123395032e-3_f64 * t35337 + 0.10215503974391481456e-3_f64 * t39748;
    t43124
}
