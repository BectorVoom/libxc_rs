//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2963/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2963<F: Float>(t13776: F, t9775: F, t46644: F, t5622: F, t5614: F, t9779: F, t40488: F, t5610: F, t13995: F, t9962: F, t2659: F, t4086: F, t816: F) -> (F, F, F, F, F, F) {
    let t48847 = t9775 * t13776;
    let t48849 = t46644 * t5622;
    let t48851 = t9779 * t5614;
    let t48853 = t40488 * t5610;
    let t48855 = t9962 * t13995;
    let t48862 = t816 * t2659 * t4086;
    (t48847, t48849, t48851, t48853, t48855, t48862)
}
