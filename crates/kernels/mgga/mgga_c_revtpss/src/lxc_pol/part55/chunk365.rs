//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 365/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk365<F: Float>(t1794: F, t482: F, t1250: F, t1042: F, t476: F, t51: F, t52: F, t475: F, t467: F, t1264: F, t1715: F, t247: F) -> (F, F, F, F, F, F) {
    let t1795 = t482 * t1794;
    let t1796 = t1795 * t1250;
    let t1797 = t1042 * t1796;
    let t1800 = t476 * t51;
    let t1802 = F::new(1.0) / t52 / t1800;
    let t1803 = t475 * t1802;
    let t1804 = t467 * t1803;
    let t1807 = t1264 * t1715;
    let t1808 = t247 * t1807;
    (t1796, t1797, t1802, t1803, t1804, t1808)
}
