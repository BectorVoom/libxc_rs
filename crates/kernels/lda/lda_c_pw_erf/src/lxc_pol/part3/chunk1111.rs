//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1111/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1111<F: Float>(t10162: F, t1325: F, t2182: F, t2188: F, t3745: F, t10169: F, t10172: F, t10173: F, t12971: F, t12975: F, t12979: F, t12982: F, t12985: F, t12988: F, t12991: F, t12996: F) -> (F, F, F) {
    let t12998 = t1325 * t10162 * t2182;
    let t12999 = F::new(8.0) / F::new(45.0) * t12998;
    let t13001 = F::new(8.0) / F::new(5.0) * t3745 * t2188;
    let t13002 = t12971 + F::new(8.0) * t10169 - t10172 + F::new(4.0) / F::new(3.0) * t10173 - t12975 + t12979 - t12982 + t12985 + t12988 + t12991 - t12996 + t12999 + t13001;
    (t12999, t13001, t13002)
}
