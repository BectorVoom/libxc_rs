//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1262/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1262<F: Float>(t11067: F, t2562: F, t8980: F, t9258: F, t8990: F, t9104: F, t11019: F, t7070: F, t11022: F, t6951: F, t11025: F, t21382: F, t11028: F, t11032: F, t11068: F, t11111: F, t11117: F, t21628: F, t2538: F, t25556: F, t2560: F, t25643: F, t25661: F, t25810: F, t25813: F, t25816: F, t3532: F, t3551: F, t7159: F, t9048: F, t9058: F, t9073: F, t9083: F, t9210: F, t9245: F, t986: F) -> (F, F, F, F, F, F, F, F) {
    let t29602 = t11067 * t2562;
    let t29627 = 8.0 * t9258 * t8980;
    let t29629 = 0.64327917994770140268e2 * t9104 * t8990;
    let t29631 = 12.0 * t7070 * t11019;
    let t29633 = 8.0 * t6951 * t11022;
    let t29635 = 0.1929837539843104208e3 * t21382 * t11025;
    let t29637 = 4.0 * t6951 * t11028;
    let t29639 = 0.32163958997385070134e2 * t7070 * t11032;
    let t29640 = -4.0 * t2538 * t11068 * t986 + 0.64327917994770140268e2 * t2560 * t29602 * t986 + 0.12865583598954028054e3 * t7159 * t11111 + 0.4138081033541872024e4 * t21628 * t11117 - 8.0 * t25643 * t3532 + 0.12865583598954028054e3 * t25556 * t3551 - 8.0 * t9245 * t9073 + 0.12865583598954028054e3 * t9210 * t9083 - 0.4155806185363551302e3 * t25816 * t9058 + 24.0 * t25810 * t9073 - 0.77193501593724168323e3 * t25661 * t9083 + 0.14035736694323150897e2 * t25813 * t9048 + t29627 - t29629 - t29631 + t29633 + t29635 + t29637 - t29639;
    (t29627, t29629, t29631, t29633, t29635, t29637, t29639, t29640)
}
