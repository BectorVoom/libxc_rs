//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 926/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk926<F: Float>(t39698: F, t39701: F, t35285: F, t35327: F, t35337: F, t39706: F, t39709: F, t39711: F, t39715: F, t39717: F, t39721: F, t39726: F, t39731: F, t39733: F, t39735: F, t39742: F, t39748: F) -> (F,) {
    let t43107 = 0.10909864661698136692e0 * t39698;
    let t43108 = 0.47896966807455234256e0 * t39701;
    let t43124 = t43107 - t43108 - 0.40911992481368012596e-1 * t39706 + 0.11918087970123395032e-3 * t35285 + 0.1702583995731913576e-4 * t39709 - 0.2553875993597870364e-4 * t39711 + 0.2553875993597870364e-4 * t39715 - 0.2553875993597870364e-4 * t39717 - 0.5107751987195740728e-4 * t39721 + 0.2553875993597870364e-4 * t39726 + 0.85129199786595678799e-5 * t39731 - 0.85129199786595678799e-5 * t39733 + 0.1702583995731913576e-4 * t39735 - 0.2553875993597870364e-3 * t39742 - 0.13242319966803772257e-3 * t35327 - 0.11918087970123395032e-3 * t35337 + 0.10215503974391481456e-3 * t39748;
    (t43124,)
}
