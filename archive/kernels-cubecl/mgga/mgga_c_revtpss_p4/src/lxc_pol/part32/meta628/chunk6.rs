//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2015/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2015<F: Float>(t103265: F, t103267: F, t106006: F, t106008: F, t106010: F, t106012: F, t106014: F, t95666: F, t98960: F, t98961: F, t98962: F, t98964: F) -> F {
    let t110385 = F::cast_from(0.40656002247428262581e-3_f64) * t106006 + F::cast_from(0.51448821741683684367e-2_f64) * t106008 - F::cast_from(0.32012600194825403606e-1_f64) * t106010 + F::cast_from(0.17149607247227894789e-2_f64) * t106012 + F::cast_from(0.16006300097412701803e-1_f64) * t106014 + t98960 - t98961 - t98962 - F::cast_from(0.60976381323476959249e-3_f64) * t98964 - t103265 - t103267 + t95666;
    t110385
}
