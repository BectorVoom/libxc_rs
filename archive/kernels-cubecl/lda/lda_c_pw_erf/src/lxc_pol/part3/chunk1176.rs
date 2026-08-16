//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1176/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1176<F: Float>(t10145: F, t11813: F, t11818: F, t11832: F, t11834: F, t11837: F, t11848: F, t11869: F, t11873: F, t11881: F, t9806: F, t9808: F, t9828: F, t9832: F, t9834: F, t9840: F, t9845: F, t9847: F, t9866: F, t9868: F) -> F {
    let t13872 = F::cast_from(0.003778333333333333_f64) * t9806 - F::cast_from(0.0012594444444444445_f64) * t9808 - t10145 - F::cast_from(0.02518888888888889_f64) * t11813 - F::cast_from(0.026448333333333334_f64) * t11818 + F::cast_from(0.002518888888888889_f64) * t9828 + F::cast_from(0.0016792592592592592_f64) * t9832 - F::cast_from(0.0006996913580246914_f64) * t9834 + F::cast_from(0.002518888888888889_f64) * t9840 - F::cast_from(0.0006297222222222223_f64) * t9845 - F::cast_from(0.005877407407407408_f64) * t9847 - F::cast_from(0.005037777777777778_f64) * t9866 - F::cast_from(0.003778333333333333_f64) * t9868 - F::cast_from(0.02267_f64) * t11832 - F::cast_from(0.0019591358024691357_f64) * t11834 + F::cast_from(0.007556666666666666_f64) * t11837 + F::cast_from(0.061712777777777776_f64) * t11848 - F::cast_from(0.02267_f64) * t11869 + F::cast_from(0.006297222222222222_f64) * t11873 + F::cast_from(0.034005_f64) * t11881;
    t13872
}
