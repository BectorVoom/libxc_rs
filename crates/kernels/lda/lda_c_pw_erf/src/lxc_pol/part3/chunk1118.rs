//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1118/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1118<F: Float>(t1318: F, t4794: F, t5230: F, t219: F, t4900: F, t4759: F, t4753: F, t5406: F, t3416: F, t3604: F, t811: F, t2017: F, t2967: F) -> (F, F, F, F, F, F) {
    let t13078 = t1318 * t4794 * t5230;
    let t13079 = F::new(16.0) / F::new(27.0) * t13078;
    let t13080 = t4900 * t219;
    let t13082 = t1318 * t13080 * t4759;
    let t13083 = F::new(8.0) / F::new(9.0) * t13082;
    let t13085 = F::new(16.0) / F::new(15.0) * t4753 * t5406;
    let t13087 = F::new(16.0) / F::new(15.0) * t3416 * t5406;
    let t13088 = t811 * t3604;
    let t13092 = F::new(16.0) / F::new(9.0) * t1318 * t2017 * t13088 * t2967;
    (t13079, t13080, t13083, t13085, t13087, t13092)
}
