//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1335/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1335<F: Float>(t11703: F, t9901: F, t11709: F, t2864: F, t16668: F, t3677: F, t3799: F, t9842: F, t10171: F, t3728: F, t11717: F, t9915: F, t15321: F, t4558: F, t9831: F, t10147: F, t10186: F, t1126: F, t1157: F, t11704: F, t11710: F, t28057: F, t28062: F, t28351: F, t28356: F, t28406: F, t28430: F, t28860: F, t2890: F, t4584: F, t653: F, t9878: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32595 = t11703 * t9901;
    let t32598 = t2864 * t11709;
    let t32601 = t3677 * t16668;
    let t32604 = t3799 * t9842;
    let t32607 = t3728 * t10171;
    let t32614 = t11717 * t9915;
    let t32618 = t15321 * t11709;
    let t32621 = t4558 * t9831;
    let t32624 = t11703 * t9915;
    let t32635 = -0.16896e-4 * t10186 * t32595 + 0.42666666666666666667e-2 * t10147 * t32598 - 0.64e1 * t28057 * t32601 + 0.64e1 * t28057 * t32604 + 0.3072e-5 * t32607 * t11704 + 0.71111111111111111111e0 * t28062 * t32604 - 0.71111111111111111111e0 * t28062 * t32601 - 0.110592e-6 * t28356 * t32614 + 0.17066666666666666667e-1 * t1157 * t28406 * t32618 + 0.110592e-6 * t28351 * t32621 + 0.36864e-7 * t28430 * t32624 - 0.10666666666666666667e-2 * t28860 * t11710 + 0.56888888888888888888e-2 * t1126 * t9878 * t653 * t32618 + 2.0 * t2890 * t4584;
    (t32595, t32598, t32601, t32604, t32614, t32618, t32621, t32624, t32635)
}
