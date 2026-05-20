//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1039/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1039<F: Float>(t3566: F, t488: F, t1276: F, t1774: F, t1209: F, t1828: F, t3736: F, t1811: F, t17306: F, t487: F, t116: F, t5876: F) -> (F, F, F, F, F, F, F) {
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    let t17986 = t1209 * t488;
    let t17987 = t3736 * t1828;
    let t17995 = t3566 * t1811;
    let t18059 = t17306 * t487;
    let t18245 = t5876 * t116;
    (t17973, t17974, t17986, t17987, t17995, t18059, t18245)
}
