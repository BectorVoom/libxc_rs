//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1164/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1164<F: Float>(t33263: F, t33265: F, t33270: F, t33275: F, t33278: F, t33261: F, t36520: F, t36521: F, t36522: F, t36523: F, t36524: F, t33284: F, t33289: F, t33292: F, t33295: F, t33298: F) -> (F, F, F, F, F, F) {
    let t36526 = 0.21720231316129303386e-4 * t33263;
    let t36527 = 0.43440462632258606772e-4 * t33265;
    let t36528 = 0.88465285289519332099e-6 * t33270;
    let t36529 = 0.10117831965157323855e-7 * t33275;
    let t36530 = 0.25002399603899953676e-2 * t33278;
    let t36531 = -t36520 + t36521 + t36522 - t36523 - t36524 + 0.97817934710145362362e-6 * t33261 + t36526 + t36527 + t36528 + t36529 + t36530;
    let t36534 = 0.75031332402051115813e-8 * t33284;
    let t36535 = 0.63350674672043801542e-5 * t33289;
    let t36536 = 0.2318836277704281739e-4 * t33292;
    let t36537 = 0.43440462632258606772e-4 * t33295;
    let t36538 = 0.43440462632258606772e-4 * t33298;
    (t36531, t36534, t36535, t36536, t36537, t36538)
}
