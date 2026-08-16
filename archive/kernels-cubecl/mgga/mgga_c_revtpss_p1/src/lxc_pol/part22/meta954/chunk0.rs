//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3197/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3197<F: Float>(t1269: F, t13126: F, t460: F, t13147: F, t1770: F, t1204: F, t17852: F, t1209: F, t1284: F, t5412: F, t17845: F, t17306: F, t3754: F) -> (F, F, F, F, F, F) {
    let t59945 = t460 * t13126 * t1269;
    let t59948 = t1770 * t13147;
    let t59987 = t1204 * t17852;
    let t60008 = t1209 * t1284 * t5412;
    let t60013 = t1204 * t17845;
    let t60019 = t17306 * t3754;
    (t59945, t59948, t59987, t60008, t60013, t60019)
}
