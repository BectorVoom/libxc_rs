//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 601/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk601<F: Float>(t1974: F, t5392: F, t1964: F, t755: F, t763: F, t5374: F, t1670: F, t1676: F, t4761: F, t591: F, t1685: F, t4762: F, t4781: F, t4787: F, t4790: F, t1966: F, t1975: F, t1979: F, t1980: F, t4698: F, t4701: F, t4708: F, t4739: F, t4747: F, t4754: F, t5365: F, t5368: F, t5373: F, t5375: F, t764: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5393 = t5392 * t1974;
    let t5396 = t1964 * t1964;
    let t5397 = 1.0 / t5396;
    let t5398 = t755 * t5397;
    let t5399 = t763 * t763;
    let t5400 = 1.0 / t5399;
    let t5401 = t5374 * t5400;
    let t5405 = t1670 * t1676;
    let t5408 = t591 * t4761;
    let t5409 = t4762 * t1685;
    let t5412 = t4781 * t1685;
    let t5415 = t591 * t4787;
    let t5416 = t4762 * t4790;
    let t5419 = -0.3109e-1 * t5365 * t764 + 2.0 * t5368 * t1975 - 2.0 * t5373 * t5375 + 1.0 * t1966 * t5393 + 0.32164683177870697974e2 * t5398 * t5401 + t4698 - t4701 + t4708 - t4739 - t4747 - 0.19751789702565206229e-1 * t4754 + 0.11696446794910408142e1 * t5405 * t1980 - 0.11696446794910408142e1 * t5408 * t5409 + 0.58482233974552040708e0 * t1979 * t5412 + 0.17315755899375863299e2 * t5415 * t5416;
    (t5393, t5396, t5397, t5398, t5399, t5400, t5401, t5405, t5408, t5409, t5412, t5415, t5416, t5419)
}
