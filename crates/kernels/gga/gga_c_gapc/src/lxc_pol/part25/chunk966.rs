//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 966/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk966<F: Float>(t2207: F, t640: F, t1645: F, t268: F, t2299: F, t830: F, t103: F, t6856: F, t11925: F, t875: F, t126: F, t17890: F, t276: F, t314: F, t442: F, t2206: F, t2250: F) -> (F, F, F, F, F, F, F, F) {
    let t22657 = t2207 * t640;
    let t22672 = t1645 * t268;
    let t22783 = t830 * t2299;
    let t22851 = t6856 * t103;
    let t22866 = t11925 * M_PI * t875;
    let t22949 = t276 * t17890 * t126;
    let t22954 = t314 * M_PI * t442;
    let t22970 = t2250 * t2206;
    (t22657, t22672, t22783, t22851, t22866, t22949, t22954, t22970)
}
