//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2186/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2186<F: Float>(t28056: F, t4248: F, t7933: F, t9593: F, t28196: F, t28198: F, t30138: F, t7003: F, t13426: F, t7735: F, t18227: F, t27137: F) -> (F, F, F, F, F, F) {
    let t108099 = F::cast_from(4.0_f64) * t4248 * t28056;
    let t108100 = t7933 * t9593;
    let t108103 = F::cast_from(4.0_f64) * t28196 * t108100 * t28198;
    let t108105 = F::cast_from(4.0_f64) * t30138 * t7003;
    let t108107 = F::cast_from(4.0_f64) * t13426 * t7735;
    let t108109 = F::cast_from(4.0_f64) * t18227 * t7735;
    let t108111 = F::cast_from(4.0_f64) * t4248 * t27137;
    (t108099, t108103, t108105, t108107, t108109, t108111)
}
