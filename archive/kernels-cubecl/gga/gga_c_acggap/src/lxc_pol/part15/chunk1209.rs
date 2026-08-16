//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1209/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1209<F: Float>(t30249: F, t32397: F, t32398: F, t32401: F, t32403: F, t32404: F, t33960: F, t33968: F, t33970: F, t33984: F, t36877: F, t36888: F, t36890: F, t36892: F, t38890: F, t38894: F, t38899: F, t38903: F) -> F {
    let t41390 = F::cast_from(0.4584375e-1_f64) * t38890 + F::cast_from(0.305625e-1_f64) * t38894 - F::cast_from(0.1528125e-1_f64) * t33960 + t36877 + t33968 + t32397 + t32398 + t32401 + t33970 + t32403 - t32404 - F::cast_from(0.90702367218671976884e-1_f64) * t30249 + F::cast_from(0.85748036236139473944e-3_f64) * t38899 - t36888 + F::cast_from(0.75475421495049964965e-2_f64) * t33984 + t36890 + F::cast_from(0.37737710747524982483e-2_f64) * t38903 + t36892;
    t41390
}
