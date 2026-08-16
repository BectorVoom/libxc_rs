//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 655/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk655(t1679: f64, t511: f64, t2136: f64, t498: f64, t615: f64, t236: f64, t7231: f64, t7230: f64, t2084: f64, t558: f64, t27: f64, t2139: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8659 = t1679 * t511;
    let t8660 = t8659 * t2136;
    let t8666 = t615 * t498;
    let t8667 = t236 * t8666;
    let t8668 = t7231 * t8667;
    let t8669 = t7230 * t8668;
    let t8671 = t2084 * t558;
    let t8672 = t27 * t8671;
    let t8673 = t2139 * t8672;
    (t8659, t8660, t8666, t8668, t8669, t8672, t8673)
}
