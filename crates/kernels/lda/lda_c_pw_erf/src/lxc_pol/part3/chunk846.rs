//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 846/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk846<F: Float>(t168: F, t2292: F, t635: F, t1905: F, t632: F, t1143: F, t781: F, t1896: F, t242: F, t5446: F, t1901: F, t245: F, t3375: F, t3378: F, t4084: F, t4087: F, t4091: F, t5451: F, t5880: F) -> F {
    let t5887 = F::cast_from(0.039794582218349216_f64) * t168 * t635 * t2292;
    let t5891 = F::cast_from(0.1675256410710088_f64) * t1905 * t632;
    let t5892 = t781 * t1143;
    let t5894 = t1896 * t242;
    let t5897 = F::cast_from(0.1675256410710088_f64) * t5446 * t242;
    let t5898 = t1901 * t632;
    let t5902 = F::cast_from(0.019897291109174608_f64) * t4087 + t4091 - F::cast_from(0.011938374665504766_f64) * t168 * t245 * t5880 - F::cast_from(0.10611888591559791_f64) * t4084 + t5887 + F::cast_from(2.657442045789236_f64) * t3378 - F::cast_from(0.5694518669548363_f64) * t3375 - t5891 - F::cast_from(0.0837628205355044_f64) * t5892 - F::cast_from(0.1675256410710088_f64) * t5894 + t5897 + F::cast_from(0.1675256410710088_f64) * t5898 - F::cast_from(0.0837628205355044_f64) * t5451 * t242;
    t5902
}
