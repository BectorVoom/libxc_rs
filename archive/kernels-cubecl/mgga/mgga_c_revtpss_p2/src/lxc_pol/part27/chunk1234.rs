//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1234/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1234<F: Float>(t2439: F, t7048: F, t780: F, t785: F, t25310: F, t25331: F, t25412: F, t93329: F, t25411: F, t25431: F, t2435: F, t25339: F) -> (F, F, F, F, F) {
    let t93382 = t2439 * t785 * t7048 * t780;
    let t93384 = t25310 * t25331;
    let t93386 = t93329 * t25412;
    let t93387 = t25411 * t93386;
    let t93389 = t25431 * t93386;
    let t93391 = t2435 * t25339;
    (t93382, t93384, t93387, t93389, t93391)
}
