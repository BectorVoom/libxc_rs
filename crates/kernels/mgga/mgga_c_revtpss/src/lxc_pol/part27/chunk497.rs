//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 497/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk497<F: Float>(t2479: F, t2488: F, t2648: F, t2653: F, t2656: F, t2666: F, t2672: F, t2678: F, t2686: F, t2691: F, t2759: F, t825: F, t851: F) -> F {
    let t2760 = F::new(0.42874018118069736972e-2) * t851 * t2479 - F::new(0.25410001404642664112e-4) * t2488 - F::new(0.21437009059034868486e-3) * t825 * t2648 + F::new(0.80031500487063509015e-2) * t2653 - F::new(0.85748036236139473944e-3) * t851 * t2656 + F::new(0.14291339372689912324e-4) * t2666 - t2672 - F::new(0.10164000561857065645e-3) * t2678 + t2686 + t2691 + t2759;
    t2760
}
