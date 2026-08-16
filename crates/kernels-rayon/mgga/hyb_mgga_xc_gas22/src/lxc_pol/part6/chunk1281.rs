//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1281/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1281(t1816: f64, t3806: f64, t1874: f64, t1877: f64, t10087: f64, t551: f64, t3814: f64, t545: f64, t668: f64, t1796: f64, t9838: f64, t1230: f64, t2970: f64, t2974: f64, t7847: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27719 = t3806 * t1816;
    let t27721 = t3806 * t1874;
    let t27723 = t3806 * t1877;
    let t27725 = t10087 * t551;
    let t27728 = t668 * t3814 * t545;
    let t27732 = t9838 * t1796;
    let t27741 = t2970 * t7847 * t1230 * t2974;
    (t27719, t27721, t27723, t27725, t27728, t27732, t27741)
}
