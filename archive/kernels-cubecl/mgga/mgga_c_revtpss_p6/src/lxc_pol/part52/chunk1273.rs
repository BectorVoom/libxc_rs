//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1273/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1273<F: Float>(t32633: F, t7898: F, t121593: F, t2014: F, t7900: F, t28189: F, t8698: F, t32630: F, t125428: F, t2107: F, t125950: F, t1843: F, t2052: F, t27830: F, t32107: F, t32109: F, t32112: F, t32609: F, t5517: F, t7357: F, t7883: F, t8463: F, t8627: F) -> F {
    let t128898 = t7898 * t32633;
    let t128903 = F::cast_from(3.0_f64) * t2014 * t121593 * t7900;
    let t128904 = t8698 * t28189;
    let t128906 = F::cast_from(3.0_f64) * t7898 * t32630;
    let t128910 = t2014 * t2107 * t125428;
    let t128911 = -t1843 * t32609 - t2052 * t27830 - t5517 * t8627 - t7357 * t7883 - t125950 - t128898 + t128903 - t128904 + t128906 - t128910 - t32107 - t32109 - t32112 - t8463;
    t128911
}
