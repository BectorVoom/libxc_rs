//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1696/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1696<F: Float>(t2435: F, t3900: F, t212: F, t4066: F, t1358: F, t689: F, t3896: F, t9303: F, t1419: F, t785: F, t2439: F, t784: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9632 = t2435 * t3900;
    let t9634 = t212 * t4066;
    let t9635 = t9634 * t1358;
    let t9636 = t689 * t9635;
    let t9639 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t3896;
    let t9640 = t785 * t1419;
    let t9641 = t9640 * t1358;
    let t9642 = t2439 * t9641;
    let t9644 = t784 * t784;
    (t9632, t9634, t9635, t9636, t9639, t9640, t9641, t9642, t9644)
}
