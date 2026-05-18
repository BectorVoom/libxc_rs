//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1165/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1165<F: Float>(t331: F, t4991: F, t174: F, t4697: F, t9810: F, t1950: F, t925: F, t325: F, t4685: F, t1945: F, t11: F, t12282: F, t557: F) -> (F, F, F, F, F, F, F) {
    let t13705 = t331 * t4991;
    let t13708 = t174 * t9810 * t4697;
    let t13710 = t925 * t1950;
    let t13712 = t325 * t4685;
    let t13714 = t925 * t1945;
    let t13715 = F::new(0.03199259259259259) * t13714;
    let t13717 = t11 * t557 * t12282;
    (t13705, t13708, t13710, t13712, t13714, t13715, t13717)
}
