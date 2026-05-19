//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1327/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1327<F: Float>(t10970: F, t10973: F, t10976: F, t10980: F, t10983: F, t10987: F, t10988: F, t10991: F, t10992: F, t10995: F, t143: F, t14632: F, t14911: F, t14949: F, t15217: F, t15234: F, t15276: F, t2205: F, t279: F, t296: F, t405: F, t5490: F) -> F {
    let t15281 = -F::cast_from(4.569219094474146e-06_f64) * t14911 - t10970 - F::cast_from(5.4655730795145296e-05_f64) * t10973 - F::cast_from(0.0001639671923854359_f64) * t10976 - t10980 + F::cast_from(0.0004919015771563077_f64) * t10983 + t10987 - F::cast_from(0.47896936041018434_f64) * t10988 - t10991 - F::cast_from(0.15965645347006147_f64) * t10992 - t10995 + (t14949 + t15217) * t279 + F::new(3.0) * t405 * t143 * t14632 + (t15234 + t15276) * t296 + F::new(18.0) * t5490 * t2205;
    t15281
}
