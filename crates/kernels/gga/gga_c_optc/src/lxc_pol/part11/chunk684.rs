//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 684/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk684<F: Float>(t7592: F, t7523: F, t2414: F, t777: F, t216: F, t231: F, t2417: F, t228: F, t256: F, t7501: F, t248: F, t2516: F, t243: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7593 = 0.36793333333333333333e0 * t7592;
    let t7594 = 0.93932222222222222223e0 * t7523;
    let t7609 = 0.28842592592592592592e-1 * t7523;
    let t7656 = 0.36514074074074074075e0 * t7592;
    let t7657 = 0.93011851851851851854e0 * t7523;
    let t7668 = 1.0 / t2414 / t777;
    let t7669 = t216 * t7668;
    let t7672 = 1.0 / t2417 / t231;
    let t7680 = 1.0 / t2414 / t228;
    let t7681 = t216 * t7680;
    let t7699 = 0.53272592592592592592e-1 * t7523;
    let t7713 = 0.55403703703703703703e-1 * t7523;
    let t7753 = t256 * t7501;
    let t7758 = 1.0 / t2516 / t248;
    let t7759 = t243 * t7758;
    let t7786 = 0.46308888888888888888e0 * t7592;
    (t7593, t7594, t7609, t7656, t7657, t7668, t7669, t7672, t7680, t7681, t7699, t7713, t7753, t7758, t7759, t7786)
}
