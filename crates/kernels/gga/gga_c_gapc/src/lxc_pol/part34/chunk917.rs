//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 917/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk917<F: Float>(t1086: F, t7739: F, t11990: F, t2597: F, t7503: F, t11320: F, t325: F, t11938: F, t11190: F, t11193: F, t11196: F, t11200: F, t11205: F, t11212: F, t11218: F, t11220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11991 = t1086 * t7739;
    let t11992 = t11990 * t11991;
    let t11994 = t2597 * t7503;
    let t11995 = t11990 * t11994;
    let t11997 = t325 * t11320;
    let t11998 = t11997 * t11938;
    let t12012 = 0.12147342662753799615e-3 * t11190;
    let t12013 = 0.12147342662753799615e-3 * t11193;
    let t12014 = 0.4049114220917933205e-4 * t11196;
    let t12015 = 0.86898242813537603824e-5 * t11200;
    let t12016 = 0.2530696388073708253e-5 * t11205;
    let t12017 = 0.18103800586153667463e-6 * t11212;
    let t12018 = 0.23761238269326688546e-5 * t11218;
    let t12019 = 0.86898242813537603825e-4 * t11220;
    (t11991, t11992, t11994, t11995, t11997, t11998, t12012, t12013, t12014, t12015, t12016, t12017, t12018, t12019)
}
