//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1070/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1070<F: Float>(t1469: F, t2609: F, t706: F, t1568: F, t785: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F) -> (F, F, F, F) {
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    let t14474 = t2439 * t14473;
    let t14476 = t212 * t4469;
    let t14477 = t14476 * t780;
    let t14479 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14477;
    let t14480 = t2769 * t1579;
    (t14441, t14474, t14479, t14480)
}
