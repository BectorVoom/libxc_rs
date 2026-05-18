//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 909/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk909<F: Float>(t7255: F, t8422: F, t2289: F, t35384: F, t35262: F, t35285: F, t39663: F, t39667: F, t39672: F, t39676: F, t39679: F, t39682: F, t39686: F, t39690: F, t39694: F, t39698: F, t39702: F, t39706: F, t534: F, t72: F, t7884: F) -> F {
    let t39709 = t7255 * t8422;
    let t39711 = t35384 * t2289;
    let t39713 = -F::new(0.20455996240684006296e-1) * t39663 + F::new(0.54549323308490683457e-1) * t39667 - F::new(0.79828278012425390426e-1) * t35262 + F::new(0.13637330827122670864e0) * t39672 + F::new(0.27274661654245341728e-1) * t39676 + t39679 + F::new(0.13637330827122670864e-1) * t39682 + F::new(0.6818665413561335432e-1) * t39686 - F::new(0.40911992481368012592e-1) * t39690 + F::new(0.21819729323396273382e0) * t39694 + F::new(0.54549323308490683456e-1) * t39698 - t39702 + t72 * t534 * t7884 - F::new(0.20455996240684006296e-1) * t39706 + F::new(0.59590439850616975158e-4) * t35285 + F::new(0.85129199786595678796e-5) * t39709 - F::new(0.12769379967989351819e-4) * t39711;
    t39713
}
