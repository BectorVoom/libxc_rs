//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 693/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk693<F: Float>(t164: F, t4100: F, t1198: F, t479: F, t1590: F, t458: F, t1159: F, t695: F, t4107: F, t4254: F, t4258: F, t4260: F, t4261: F, t4265: F, t4268: F) -> (F, F, F, F, F, F) {
    let t4270 = t4100 * t164;
    let t4272 = t1198 * t479;
    let t4275 = F::cast_from(0.09451622166942335_f64) * t458 * t1590;
    let t4276 = t1159 * t164;
    let t4279 = F::cast_from(0.1890324433388467_f64) * t695 * t479;
    let t4280 = t4254 + t4258 - t4260 - F::cast_from(0.09451622166942335_f64) * t4261 - t4265 - F::cast_from(0.031505407223141116_f64) * t4107 * t164 - F::cast_from(0.09451622166942335_f64) * t4268 + F::cast_from(0.09451622166942335_f64) * t4270 + F::cast_from(0.1890324433388467_f64) * t4272 + t4275 - F::cast_from(0.1890324433388467_f64) * t4276 - t4279;
    (t4270, t4272, t4275, t4276, t4279, t4280)
}
