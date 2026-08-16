//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1977/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1977(t87533: f64, t87535: f64, t87544: f64, t87546: f64, t87197: f64, t87205: f64, t87211: f64, t81750: f64, t84857: f64, t84859: f64, t87183: f64, t87185: f64, t87187: f64, t87189: f64, t87191: f64, t87193: f64, t87195: f64, t87200: f64, t87213: f64, t87216: f64, t87219: f64) -> (f64, f64, f64, f64, f64) {
    let t92560 = 0.15352717957250113407e0_f64 * t87533;
    let t92561 = 0.76763589786250567036e-1_f64 * t87535;
    let t92564 = 0.3289868133696452873e-1_f64 * t87544;
    let t92565 = 0.15352717957250113407e0_f64 * t87546;
    let t92578 = 7.0_f64 / 144.0_f64 * t87197;
    let t92580 = 0.56521858531796547194e-2_f64 * t87205;
    let t92582 = 0.13457585364713463618e-3_f64 * t87211;
    let t92586 = -t87183 / 384.0_f64 + t87185 / 96.0_f64 + t87187 / 96.0_f64 + t87189 / 96.0_f64 + t87191 / 96.0_f64 - t87193 / 768.0_f64 - t87195 / 384.0_f64 - t92578 + t87200 / 96.0_f64 - t92580 - t84857 + t84859 - 7.0_f64 / 144.0_f64 * t81750 + t92582 + 0.33643963411783659044e-4_f64 * t87213 + t87216 / 768.0_f64 + t87219 / 384.0_f64;
    (t92560, t92561, t92564, t92565, t92586)
}
