//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2860/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2860<F: Float>(t18305: F, t4186: F, t4401: F, t18576: F, t62291: F, t62302: F, t50892: F, t50893: F, t189: F, t22671: F, t606: F, t177: F, t23211: F, t762: F) -> (F, F, F, F, F, F, F) {
    let t77036 = F::new(36.0) * t4401 * t18305 * t4186;
    let t77038 = F::new(72.0) * t62291 * t18576;
    let t77039 = F::new(12.0) * t62302;
    let t77040 = F::new(3.0) * t50892;
    let t77041 = F::cast_from(0.31168546390226634765e3_f64) * t50893;
    let t77042 = t189 * t22671;
    let t77045 = F::new(12.0) * t4401 * t77042 * t606;
    let t77047 = t23211 * t177 * t762;
    (t77036, t77038, t77039, t77040, t77041, t77045, t77047)
}
