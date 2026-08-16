//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1174/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1174<F: Float>(t13804: F, t4506: F, t4515: F, t4516: F, t954: F, t3604: F, t4521: F, t951: F, t13771: F, t13773: F, t12414: F, t4523: F) -> (F, F, F, F, F, F, F) {
    let t13807 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t4515 * t13804;
    let t13808 = t4516 * t954;
    let t13811 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4506 * t4515 * t13808;
    let t13812 = t4521 * t3604;
    let t13813 = t4516 * t951;
    let t13816 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t4506 * t13812 * t13813;
    let t13819 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t13771 * t4515 * t13773;
    let t13821 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12414 * t4523;
    (t13807, t13808, t13811, t13813, t13816, t13819, t13821)
}
