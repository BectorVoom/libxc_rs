//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 990/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk990<F: Float>(t27383: F, t51792: F, t25207: F, t51806: F, t2257: F, t890: F, t10818: F, t27159: F, t2832: F, t605: F, t2408: F, t2411: F, t14365: F, t2430: F, t775: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t92762 = t27383 * t51792;
    let t92765 = t25207 * t51806;
    let t92768 = t2257 * t890;
    let t92772 = t27159 * t10818;
    let t92779 = t605 * t2832;
    let t92783 = t605 * t2408;
    let t92790 = t2411 * t605;
    let t92791 = t92790 * t14365;
    let t92795 = t605 * t2430;
    let t92799 = t2257 * t775;
    let t92806 = t605 * t2394;
    (t92762, t92765, t92768, t92772, t92779, t92783, t92791, t92795, t92799, t92806)
}
