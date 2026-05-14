//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 915/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk915<F: Float>(t119836: F, t119839: F, t2684: F, t8486: F, t25410: F, t7063: F, t8471: F, t2801: F, t125: F, t2769: F, t1032: F, t1949: F, t867: F, t786: F, t25296: F, t243: F, t257: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t119840 = t119836 * t119839;
    let t119842 = t8486 * t2684;
    let t119843 = 0.49169913065300780973e-2 * t119842;
    let t119849 = t7063 * t8471 * t25410;
    let t119850 = t119849 * t2801;
    let t119852 = t125 * t2769;
    let t119857 = t1949 * t1032;
    let t119858 = t119857 * t867;
    let t119859 = t786 * t119858;
    let t119860 = t119859 * t25296;
    let t119867 = t243 * t257;
    (t119840, t119843, t119849, t119850, t119852, t119857, t119858, t119859, t119860, t119867)
}
