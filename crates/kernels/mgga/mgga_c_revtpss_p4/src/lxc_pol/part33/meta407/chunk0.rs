//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1458/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1458<F: Float>(t13126: F, t487: F, t460: F, t3754: F, t5219: F, t3566: F, t488: F, t1276: F, t1774: F, t1209: F, t1811: F, t1269: F, t1770: F) -> (F, F, F, F, F, F, F) {
    let t17948 = t13126 * t487;
    let t17949 = t460 * t17948;
    let t17958 = t5219 * t3754;
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    let t17986 = t1209 * t488;
    let t17995 = t3566 * t1811;
    let t18005 = t1770 * t1269;
    (t17949, t17958, t17973, t17974, t17986, t17995, t18005)
}
