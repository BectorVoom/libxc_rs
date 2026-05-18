//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1221/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1221<F: Float>(t25207: F, t51775: F, t41161: F, t27383: F, t51792: F, t51806: F, t2257: F, t890: F, t10818: F, t27159: F, t2411: F, t25435: F) -> (F, F, F, F, F, F, F) {
    let t92753 = t25207 * t51775;
    let t92759 = t25207 * t41161;
    let t92762 = t27383 * t51792;
    let t92765 = t25207 * t51806;
    let t92768 = t2257 * t890;
    let t92772 = t27159 * t10818;
    let t92775 = t25435 * t2411;
    (t92753, t92759, t92762, t92765, t92768, t92772, t92775)
}
