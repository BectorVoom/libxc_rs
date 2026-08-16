//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 392/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk392(t2350: f64, t797: f64, t2347: f64, t262: f64, t2100: f64, t2103: f64, t851: f64, t854: f64, t2115: f64, t2118: f64, t2099: f64, t2108: f64, t2114: f64, t2122: f64, t2348: f64) -> (f64, f64, f64) {
    let t2351 = t797 * t2350;
    let t2353 = t262 * t2347;
    let t2354 = t2100 * t2353;
    let t2356 = t262 * t2350;
    let t2357 = t2103 * t2356;
    let t2359 = t851 * t2347;
    let t2361 = t854 * t2350;
    let t2363 = t2115 * t2353;
    let t2365 = t2118 * t2356;
    let t2367 = -0.99785347515531738034e-2_f64 * t2348 + 0.14967802127329760705e-1_f64 * t2351 + t2099 + 0.34093327067806677162e-2_f64 * t2354 - 0.45457769423742236216e-2_f64 * t2357 - t2108 - 0.33190385262651453347e-3_f64 * t2359 + 0.39828462315181744016e-3_f64 * t2361 + t2114 + 0.9072038638458063915e-4_f64 * t2363 - 0.10584045078201074568e-3_f64 * t2365 - t2122;
    (t2353, t2356, t2367)
}
