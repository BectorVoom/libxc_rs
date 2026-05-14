//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1309/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1309<F: Float>(t1864: F, t415: F, t6966: F, t1772: F, t2447: F, t4830: F, t32947: F, t7218: F, t1790: F, t32935: F, t7261: F, t7268: F, t112591: F, t116220: F, t116251: F, t116276: F, t116756: F, t32921: F, t32948: F, t32990: F, t33002: F, t34182: F, t9649: F, t9652: F, t9664: F, t9672: F) -> (F, F, F) {
    let t116787 = t415 * t1864 * t6966;
    let t116790 = t4830 * t2447 * t1772;
    let t116793 = t32947 * t7218;
    let t116806 = t7261 * t32935 * t7268 * t1790;
    let t116819 = -0.88437037037037037034e-2 * t116787 + 0.20833333333333333334e-1 * t116790 * t9672 - 0.21444444444444444446e-1 * t116793 * t9652 + 0.20833333333333333334e-1 * t116790 * t9652 + 0.17972642500000000001e-2 * t112591 * t116276 - 0.23280625000000000001e-2 * t33002 * t116251 - 0.20833333333333333334e-1 * t32990 * t34182 - 0.20833333333333333334e-1 * t9664 * t116806 - 0.10416666666666666667e-1 * t9664 * t116756 + 0.120625e-1 * t9649 * t116220 - 0.8041666666666666667e-2 * t32948 * t34182 - 0.8041666666666666667e-2 * t32921 * t34182 - 0.8041666666666666667e-2 * t9649 * t116806;
    (t116787, t116790, t116819)
}
