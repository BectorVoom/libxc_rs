//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1324/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1324<F: Float>(t22860: F, t94493: F, t22854: F, t7271: F, t22956: F, t7264: F, t22822: F, t22815: F, t108604: F, t108608: F, t108623: F, t108625: F, t108627: F, t108629: F, t94569: F, t94571: F, t98285: F) -> F {
    let t114573 = t94493 * t22860;
    let t114575 = t7271 * t22854;
    let t114577 = t7264 * t22956;
    let t114584 = t7271 * t22822;
    let t114586 = t7271 * t22815;
    let t114588 = -F::new(0.85748036236139473944e-4) * t108604 - F::new(0.30492001685571196935e-3) * t108608 - F::new(0.25724410870841842183e-2) * t114573 + F::new(0.25724410870841842184e-1) * t114575 - F::new(0.42874018118069736972e-3) * t114577 - t94569 - t94571 - F::new(0.1084295579938911763e-3) * t98285 + F::new(0.42874018118069736972e-4) * t108623 + F::new(0.15246000842785598468e-2) * t108625 - F::new(0.12004725073059526352e0) * t108627 + F::new(0.24009450146119052704e-1) * t108629 - F::new(0.17149607247227894789e-2) * t114584 - F::new(0.51448821741683684367e-1) * t114586;
    t114588
}
