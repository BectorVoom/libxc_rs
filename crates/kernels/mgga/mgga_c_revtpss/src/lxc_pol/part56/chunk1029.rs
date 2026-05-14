//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1029/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1029<F: Float>(t33515: F, t33516: F, t5291: F, t12915: F, t247: F, t33405: F, t34934: F, t3736: F, t482: F, t31993: F, t33524: F, t5377: F, t42859: F, t13038: F, t1794: F, t8931: F) -> (F, F, F, F, F, F, F) {
    let t131556 = t33515 * t33516 * t5291;
    let t131576 = t33405 * t247 * t12915 * t34934;
    let t131578 = t482 * t3736;
    let t131584 = t33524 * t31993 * t5377;
    let t131591 = t42859 * t3736;
    let t131592 = t131591 * t13038;
    let t131594 = t8931 * t1794;
    (t131556, t131576, t131578, t131584, t131591, t131592, t131594)
}
