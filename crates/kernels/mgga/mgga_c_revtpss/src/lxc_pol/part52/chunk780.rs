//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 780/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk780<F: Float>(t11199: F, t1981: F, t7143: F, t3336: F, t7177: F, t11108: F, t1989: F, t2411: F, t33: F) -> (F, F, F, F) {
    let t25698 = t1981 * t11199;
    let t25699 = t25698 * t7143;
    let t25709 = t7177 * t3336;
    let t25713 = t1989 * t11108;
    let t25759 = t2411 * t33;
    (t25699, t25709, t25713, t25759)
}
