//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1041/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1041(t11205: f64, t11212: f64, t11218: f64, t11220: f64, t11225: f64, t11229: f64, t11231: f64, t11183: f64, t11186: f64, t12012: f64, t12013: f64, t12014: f64, t12015: f64) -> f64 {
    let t12016 = 0.2530696388073708253e-5_f64 * t11205;
    let t12017 = 0.18103800586153667463e-6_f64 * t11212;
    let t12018 = 0.23761238269326688546e-5_f64 * t11218;
    let t12019 = 0.86898242813537603825e-4_f64 * t11220;
    let t12020 = 0.86898242813537603825e-4_f64 * t11225;
    let t12021 = 0.2530696388073708253e-5_f64 * t11229;
    let t12022 = 0.3475929712541504153e-3_f64 * t11231;
    let t12023 = 0.54311401758461002391e-5_f64 * t11183 + 0.54311401758461002391e-5_f64 * t11186 - t12012 - t12013 - t12014 + t12015 + t12016 - t12017 + t12018 - t12019 - t12020 + t12021 + t12022;
    t12023
}
