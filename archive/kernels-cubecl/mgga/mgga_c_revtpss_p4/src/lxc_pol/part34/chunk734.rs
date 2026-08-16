//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 734/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk734<F: Float>(t116: F, t1936: F, t38: F, t4173: F, t1497: F, t84: F, t77: F, t1470: F, t603: F, t1469: F, t6968: F, t6971: F) -> (F, F, F, F, F) {
    let t7330 = t116 * t1936;
    let t7702 = t4173 * t38;
    let t7705 = t84 * t1497;
    let t7706 = t77 * t7705;
    let t7709 = t603 * t1470;
    let t7714 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6968 * t1469 + t6971;
    (t7330, t7702, t7706, t7709, t7714)
}
