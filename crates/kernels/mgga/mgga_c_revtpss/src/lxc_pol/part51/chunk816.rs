//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 816/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk816<F: Float>(t31812: F, t8471: F, t886: F, t2718: F, t2769: F, t231: F, t836: F, t1949: F, t7048: F, t8650: F, t25386: F, t8485: F, t817: F, t251: F, t31805: F) -> (F, F, F, F, F, F, F) {
    let t31814 = t31812 * t8471 * t886;
    let t31817 = t2769 * t2718;
    let t31819 = t8471 * t836 * t231;
    let t31820 = t31817 * t31819;
    let t31824 = t8650 * t1949 * t7048;
    let t31827 = t25386 * t8485;
    let t31828 = t31827 * t817;
    let t31829 = 0.1859366460452550541e-4 * t31828;
    let t31830 = t31805 * t251;
    (t31814, t31817, t31820, t31824, t31827, t31829, t31830)
}
