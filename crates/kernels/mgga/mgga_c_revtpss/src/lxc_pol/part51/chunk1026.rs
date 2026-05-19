//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1026/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1026<F: Float>(t233: F, t240: F, t27: F, t119833: F, t124: F, t257: F, t10779: F, t775: F, t2684: F, t8486: F, t25410: F, t7063: F, t8471: F) -> (F, F, F, F, F, F, F) {
    let t119835 = t233 * t27 * t240;
    let t119836 = t119833 * t119835;
    let t119837 = t124 * t257;
    let t119839 = t10779 * t119837 * t775;
    let t119840 = t119836 * t119839;
    let t119842 = t8486 * t2684;
    let t119843 = F::cast_from(0.49169913065300780973e-2_f64) * t119842;
    let t119849 = t7063 * t8471 * t25410;
    (t119835, t119836, t119837, t119839, t119840, t119843, t119849)
}
