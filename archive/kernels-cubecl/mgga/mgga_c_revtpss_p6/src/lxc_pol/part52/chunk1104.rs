//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1104/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1104<F: Float>(t34036: F, t34245: F, t34274: F, t34330: F, t3: F, t2042: F, t8118: F, t2113: F, t7950: F, t7953: F, t1916: F, t8731: F, param_d: F) -> (F, F, F, F, F, F, F) {
    let t34332 = t34036 + t34245 + t34274 + t34330;
    let t34333 = t3 * t34332;
    let t34341 = param_d * t34332;
    let t34346 = F::cast_from(3.0_f64) * t8118 * t2042;
    let t34348 = F::cast_from(6.0_f64) * t2113 * t7950;
    let t34350 = F::cast_from(3.0_f64) * t2113 * t7953;
    let t34358 = F::cast_from(6.0_f64) * t1916 * t8731;
    (t34332, t34333, t34341, t34346, t34348, t34350, t34358)
}
