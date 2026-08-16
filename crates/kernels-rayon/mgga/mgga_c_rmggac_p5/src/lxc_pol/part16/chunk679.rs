//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 679/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk679(t678: f64, t9826: f64, t1737: f64, t649: f64, t27: f64, t7273: f64, t1763: f64, t7263: f64, t570: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9827 = t9826 * t678;
    let t9830 = t649 * t1737;
    let t9831 = t27 * t9830;
    let t9832 = t7273 * t9831;
    let t9834 = t649 * t1763;
    let t9835 = t27 * t9834;
    let t9836 = t7263 * t9835;
    let t9843 = t570 * t615;
    (t9827, t9831, t9832, t9835, t9836, t9843)
}
