//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1971/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1971<F: Float>(t560: F, t9655: F, t1389: F, t268: F, t10115: F, t555: F, t4146: F, t198: F, t775: F, t11821: F, t65: F, t2246: F, t4171: F) -> (F, F, F, F, F, F, F) {
    let t46361 = F::cast_from(1.0_f64) / t9655 / t560;
    let t46808 = t1389 * t268;
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = F::cast_from(1.0_f64) / t47671;
    let t50080 = t198 * t775;
    let t53321 = t65 * t11821;
    let t60221 = t4171 * t2246;
    (t46361, t46808, t47567, t47672, t50080, t53321, t60221)
}
