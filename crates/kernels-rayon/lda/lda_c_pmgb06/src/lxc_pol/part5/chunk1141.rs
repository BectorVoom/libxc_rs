//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1141/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1141(t20693: f64, t19756: f64, t19760: f64, t19764: f64, t19768: f64, t19772: f64, t19774: f64, t19776: f64, t19780: f64, t19784: f64, t19788: f64, t19793: f64, t19796: f64, t19799: f64, t19804: f64, t19807: f64, t19811: f64, t19814: f64, t19816: f64, t19818: f64, t9215: f64) -> (f64, f64) {
    let t20694 = t20693 / 45.0_f64;
    let t20715 = -0.04534_f64 * t19756 + 0.003778333333333333_f64 * t19760 + 0.007556666666666666_f64 * t19764 - 0.04534_f64 * t19768 + 0.06801_f64 * t19772 - 0.0012594444444444445_f64 * t19774 + 0.003778333333333333_f64 * t19776 - 0.011335_f64 * t19780 - 0.02267_f64 * t19784 + 0.04534_f64 * t19788 - 0.02518888888888889_f64 * t19793 + 0.005597530864197531_f64 * t19796 + 0.012594444444444445_f64 * t19799 - 0.003778333333333333_f64 * t19804 + 0.0012594444444444445_f64 * t19807 - 0.0019591358024691357_f64 * t9215 - 0.02267_f64 * t19811 + 0.006297222222222222_f64 * t19814 - 0.0006996913580246914_f64 * t19816 - 0.0006297222222222223_f64 * t19818;
    (t20694, t20715)
}
