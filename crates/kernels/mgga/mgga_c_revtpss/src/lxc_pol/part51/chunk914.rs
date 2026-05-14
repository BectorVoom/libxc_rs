//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 914/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk914<F: Float>(t119825: F, t2466: F, t119823: F, t25377: F, t676: F, t7048: F, t32474: F, t1032: F, t7063: F, t233: F, t240: F, t27: F, t124: F, t257: F, t10779: F, t775: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119826 = t119825 * t2466;
    let t119827 = t119823 * t119826;
    let t119830 = t25377 * t676 * t7048;
    let t119831 = t32474 * t119830;
    let t119833 = t7063 * t1032;
    let t119835 = t233 * t27 * t240;
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    (t119826, t119827, t119830, t119831, t119833, t119835, t119836, t119837, t119839)
}
