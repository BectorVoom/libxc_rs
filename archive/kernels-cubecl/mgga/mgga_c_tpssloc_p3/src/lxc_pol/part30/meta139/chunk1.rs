//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 768/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk768<F: Float>(t515: F, t518: F, t215: F, t2559: F, t535: F, t1314: F, t782: F) -> (F, F, F, F) {
    let t3704 = F::cast_from(1.0_f64) / t515;
    let t3711 = F::cast_from(1.0_f64) / t518;
    let t3725 = F::cast_from(0.64814814814814814813e-2_f64) * t2559 * t535 * t215;
    let t3726 = t782 * t1314;
    (t3704, t3711, t3725, t3726)
}
