//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1143/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1143<F: Float>(t3416: F, t7680: F, t1318: F, t2334: F, t4688: F, t4758: F, t21051: F, t21056: F, t21057: F, t21058: F, t21059: F, t21060: F, t21064: F, t21066: F, t21067: F, t21069: F, t21071: F) -> (F, F, F) {
    let t21073 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3416 * t7680;
    let t21077 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t1318 * t4758 * t4688 * t2334;
    let t21078 = t21051 - t21056 + t21057 + t21058 + t21059 - t21060 + t21064 - t21066 + t21067 + t21069 - t21071 - t21073 + t21077;
    (t21073, t21077, t21078)
}
