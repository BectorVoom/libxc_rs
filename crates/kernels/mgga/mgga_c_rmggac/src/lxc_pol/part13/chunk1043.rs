//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1043/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1043<F: Float>(t38837: F, t38853: F, t38857: F, t38860: F, t38863: F, t38869: F, t34799: F, t37221: F, t37222: F, t37223: F, t38822: F, t38826: F, t38833: F, t38841: F, t38846: F, t38850: F, t38866: F) -> F {
    let t42755 = F::new(0.1440846329149835838e-2) * t38837;
    let t42759 = F::new(0.20496175532535769482e-3) * t38853;
    let t42760 = F::new(0.1440846329149835838e-2) * t38857;
    let t42761 = F::new(0.1440846329149835838e-2) * t38860;
    let t42762 = F::new(0.1440846329149835838e-2) * t38863;
    let t42764 = F::new(0.20496175532535769482e-3) * t38869;
    let t42765 = -F::new(0.20496175532535769482e-3) * t38822 + F::new(0.60975299583150056624e-3) * t38826 - t37221 + t37222 - t37223 - F::new(0.2881692658299671676e-2) * t34799 + F::new(0.60975299583150056624e-3) * t38833 + t42755 - F::new(0.86737941314158990616e-4) * t38841 - F::new(0.86737941314158990616e-4) * t38846 - F::new(0.1440846329149835838e-2) * t38850 - t42759 + t42760 + t42761 + t42762 + F::new(0.72042316457491791901e-3) * t38866 - t42764;
    t42765
}
