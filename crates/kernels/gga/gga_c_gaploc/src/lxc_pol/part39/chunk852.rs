//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 852/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk852<F: Float>(t42967: F, t10736: F, t29277: F, t7064: F, t10635: F, t2554: F, t1841: F, t3487: F, t734: F, t9641: F, t40588: F, t40591: F, t40596: F, t40599: F, t40602: F, t13194: F, t29439: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t42968 = 0.38452631150711121417e-2 * t42967;
    let t42970 = t7064 * t29277 * t10736;
    let t42971 = 0.12817543716903707139e-2 * t42970;
    let t42973 = t7064 * t10635 * t2554;
    let t42974 = 0.64087718584518535698e-3 * t42973;
    let t42978 = 0.85450291446024714263e-3 * t1841 * t9641 * t3487 * t734;
    let t42980 = 0.1922631557535556071e-2 * t40588;
    let t42981 = 0.4486140300916297499e-2 * t40591;
    let t42982 = 0.7690526230142224284e-2 * t40596;
    let t42983 = 0.3845263115071112142e-2 * t40599;
    let t42984 = 0.1281754371690370714e-2 * t40602;
    let t42985 = t29439 * t13194;
    (t42968, t42971, t42974, t42978, t42980, t42981, t42982, t42983, t42984, t42985)
}
