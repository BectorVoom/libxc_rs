//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2118/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118<F: Float>(t25082: F, t49582: F, t8717: F, t2014: F, t25089: F, t28172: F, t27154: F, t95088: F, t26089: F, t5542: F, t2322: F, t28043: F) -> (F, F, F, F, F) {
    let t98458 = F::cast_from(3.0_f64) * t25082 * t8717 * t49582;
    let t98461 = F::cast_from(3.0_f64) * t2014 * t28172 * t25089;
    let t98463 = F::cast_from(6.0_f64) * t95088 * t27154;
    let t98467 = t2014 * t26089 * t5542;
    let t98472 = F::cast_from(4.0_f64) * t2322 * t28043;
    (t98458, t98461, t98463, t98467, t98472)
}
