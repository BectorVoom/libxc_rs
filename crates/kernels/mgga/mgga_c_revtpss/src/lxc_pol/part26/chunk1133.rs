//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1133/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1133<F: Float>(t25410: F, t93341: F, t25374: F, t93169: F, t25759: F, t51806: F, t27799: F, t50066: F, t51792: F, t51775: F, t1113: F, t2411: F) -> (F, F, F, F, F, F, F) {
    let t93374 = t93341 * t25410;
    let t93377 = t93169 * t25374;
    let t94228 = t25759 * t51806;
    let t94231 = t27799 * t50066;
    let t94234 = t27799 * t51792;
    let t94240 = t25759 * t51775;
    let t94245 = t2411 * t1113;
    (t93374, t93377, t94228, t94231, t94234, t94240, t94245)
}
