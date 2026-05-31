//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 861/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk861<F: Float>(t267: F, t3439: F, t6225: F, t7496: F, t7497: F, t7499: F, t7500: F, t7501: F, t7502: F, t7503: F, t7504: F, t7505: F, t7507: F, t7509: F, t7511: F, t7512: F, t7517: F, t7518: F, t8025: F) -> F {
    let t8028 = t7496 - t7497 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6225 + t7499 - t8025 * t267 / F::cast_from(15.0_f64) - t7500 - t7501 + t7502 + t7503 - t7504 - t7505 - t7507 + t7509 + t7511 + t3439 + t7512 - t7517 + t7518;
    t8028
}
