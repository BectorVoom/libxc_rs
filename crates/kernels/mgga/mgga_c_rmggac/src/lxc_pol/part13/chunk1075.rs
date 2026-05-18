//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1075/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1075<F: Float>(t40479: F, t40505: F, t35742: F, t35744: F, t35752: F, t35766: F, t40481: F, t40489: F, t40491: F, t40493: F, t40495: F, t40502: F, t40507: F, t40509: F, t40511: F, t40513: F, t40516: F) -> F {
    let t43433 = F::new(0.39726959900411316772e-4) * t40479;
    let t43440 = F::new(0.39726959900411316772e-4) * t40505;
    let t43446 = F::new(0.60975299583150056624e-3) * t35742 + F::new(0.60975299583150056624e-3) * t35744 + F::new(0.47896966807455234256e0) * t35752 + F::new(0.15965655602485078085e0) * t35766 + t43433 + F::new(0.1702583995731913576e-4) * t40481 + F::new(0.2727466165424534173e-1) * t40489 - F::new(0.40911992481368012596e-1) * t40491 + F::new(0.5454932330849068346e-1) * t40493 - F::new(0.85129199786595678799e-5) * t40495 + F::new(0.1702583995731913576e-4) * t40502 - t43440 - F::new(0.1702583995731913576e-4) * t40507 - F::new(0.85129199786595678799e-5) * t40509 + F::new(0.5107751987195740728e-4) * t40511 + F::new(0.2553875993597870364e-4) * t40513 - F::new(0.95793933614910468511e0) * t40516;
    t43446
}
