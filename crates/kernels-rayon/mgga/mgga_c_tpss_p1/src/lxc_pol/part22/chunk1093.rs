//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1093/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1093(t11845: f64, t11848: f64, t11850: f64, t11853: f64, t11892: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11911: f64, t11913: f64, t11916: f64, t11919: f64, t11922: f64, t11925: f64, t11965: f64, t9183: f64, t9192: f64, t9194: f64, t9196: f64, t9297: f64, t9306: f64) -> f64 {
    let t11967 = -t9297 + 0.18257037037037037037e-1_f64 * t9183 + 0.18257037037037037037e0_f64 * t9192 - 0.54771111111111111111e-1_f64 * t9194 - 0.10954222222222222222e0_f64 * t9196 - t11845 + 0.82156666666666666667e-1_f64 * t11848 + 0.91285185185185185185e-1_f64 * t11850 - t9306 + 0.142419375e1_f64 * t11853 + t11892 - 0.19931111111111111111e0_f64 * t11896 + 0.17938e1_f64 * t11899 + 0.11958666666666666667e1_f64 * t11904 + 0.59793333333333333334e0_f64 * t11908 - t11911 - 0.54771111111111111112e-1_f64 * t11913 - 0.27385555555555555556e-1_f64 * t11916 - 0.16431333333333333333e0_f64 * t11919 + 0.32862666666666666666e0_f64 * t11922 + 0.16431333333333333333e0_f64 * t11925 + t11965;
    t11967
}
