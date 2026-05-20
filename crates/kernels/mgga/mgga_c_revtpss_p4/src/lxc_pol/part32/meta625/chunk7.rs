//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1985/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1985<F: Float>(t102468: F, t108508: F, t108510: F, t108512: F, t108514: F, t108516: F, t108518: F, t108520: F, t108522: F, t108524: F, t108526: F, t108528: F) -> F {
    let t109777 = -t102468 + F::cast_from(0.34299214494455789578e-2_f64) * t108508 - F::cast_from(0.17149607247227894789e-2_f64) * t108510 + F::cast_from(0.17149607247227894789e-2_f64) * t108512 + F::cast_from(0.51448821741683684367e-2_f64) * t108514 - F::cast_from(0.32012600194825403606e-1_f64) * t108516 - F::cast_from(0.51448821741683684367e-2_f64) * t108518 - F::cast_from(0.17149607247227894789e-1_f64) * t108520 + F::cast_from(0.34299214494455789578e-2_f64) * t108522 + F::cast_from(0.40656002247428262581e-3_f64) * t108524 + F::cast_from(0.34299214494455789578e-2_f64) * t108526 - F::cast_from(0.85748036236139473944e-3_f64) * t108528;
    t109777
}
