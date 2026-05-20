//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1264/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1264<F: Float>(t28760: F, t8634: F, t34167: F, t649: F, t32633: F, t7898: F, t121593: F, t2014: F, t7900: F, t28189: F, t8698: F, t32630: F) -> (F, F, F, F, F, F) {
    let t128882 = F::new(2.0) * t8634 * t28760;
    let t128891 = t649 * t34167;
    let t128898 = t7898 * t32633;
    let t128903 = F::new(3.0) * t2014 * t121593 * t7900;
    let t128904 = t8698 * t28189;
    let t128906 = F::new(3.0) * t7898 * t32630;
    (t128882, t128891, t128898, t128903, t128904, t128906)
}
