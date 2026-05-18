//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 864/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk864<F: Float>(t38946: F, t38968: F, t38986: F, t38998: F, t39023: F, t39025: F, t39031: F, t39233: F, t39250: F, t39252: F, t39255: F, t39264: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42788 = F::new(0.1454648621559751559e0) * t38946;
    let t42794 = F::new(0.49658699875514145965e-4) * t38968;
    let t42800 = F::new(0.11918087970123395032e-3) * t38986;
    let t42806 = F::new(0.11918087970123395032e-3) * t38998;
    let t42820 = F::new(0.36366215538993788974e-1) * t39023;
    let t42821 = F::new(0.10909864661698136692e0) * t39025;
    let t42823 = F::new(0.10909864661698136692e0) * t39031;
    let t42886 = F::new(0.39726959900411316772e-4) * t39233;
    let t42890 = F::new(0.11918087970123395032e-3) * t39250;
    let t42891 = F::new(0.11918087970123395032e-3) * t39252;
    let t42892 = F::new(0.60975299583150056624e-3) * t39255;
    let t42899 = F::new(0.39726959900411316772e-4) * t39264;
    (t42788, t42794, t42800, t42806, t42820, t42821, t42823, t42886, t42890, t42891, t42892, t42899)
}
