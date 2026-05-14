//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 645/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk645<F: Float>(t236: F, t9988: F, t7231: F, t7230: F, t530: F, t8817: F, t1743: F, t645: F, t903: F, t1734: F, t665: F, t739: F, t2379: F, t4985: F, t1707: F, t2024: F, t6522: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9989 = t236 * t9988;
    let t9990 = t7231 * t9989;
    let t9991 = t7230 * t9990;
    let t9992 = 0.1064114997332445985e-4 * t9991;
    let t9997 = t530 * t8817;
    let t9998 = 0.4726e1 * t9997;
    let t9999 = t645 * t1743;
    let t10000 = t903 * t9999;
    let t10001 = 0.44903406381989282115e-1 * t10000;
    let t10002 = t665 * t1734;
    let t10003 = t739 * t10002;
    let t10004 = 0.59871208509319042821e-1 * t10003;
    let t10005 = t4985 * t2379;
    let t10006 = 0.11974241701863808564e0 * t10005;
    let t10007 = t665 * t1707;
    let t10008 = t903 * t10007;
    let t10009 = 0.35922725105591425692e0 * t10008;
    let t10010 = t2024 * t6522;
    (t9990, t9992, t9998, t9999, t10001, t10002, t10004, t10006, t10007, t10009, t10010)
}
