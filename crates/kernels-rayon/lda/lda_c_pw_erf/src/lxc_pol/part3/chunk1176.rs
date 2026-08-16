//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1176/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1176(t10145: f64, t11813: f64, t11818: f64, t11832: f64, t11834: f64, t11837: f64, t11848: f64, t11869: f64, t11873: f64, t11881: f64, t9806: f64, t9808: f64, t9828: f64, t9832: f64, t9834: f64, t9840: f64, t9845: f64, t9847: f64, t9866: f64, t9868: f64) -> f64 {
    let t13872 = 0.003778333333333333_f64 * t9806 - 0.0012594444444444445_f64 * t9808 - t10145 - 0.02518888888888889_f64 * t11813 - 0.026448333333333334_f64 * t11818 + 0.002518888888888889_f64 * t9828 + 0.0016792592592592592_f64 * t9832 - 0.0006996913580246914_f64 * t9834 + 0.002518888888888889_f64 * t9840 - 0.0006297222222222223_f64 * t9845 - 0.005877407407407408_f64 * t9847 - 0.005037777777777778_f64 * t9866 - 0.003778333333333333_f64 * t9868 - 0.02267_f64 * t11832 - 0.0019591358024691357_f64 * t11834 + 0.007556666666666666_f64 * t11837 + 0.061712777777777776_f64 * t11848 - 0.02267_f64 * t11869 + 0.006297222222222222_f64 * t11873 + 0.034005_f64 * t11881;
    t13872
}
