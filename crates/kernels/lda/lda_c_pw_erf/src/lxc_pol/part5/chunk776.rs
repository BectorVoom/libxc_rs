//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 776/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk776<F: Float>(t2630: F, t443: F, t2634: F, t450: F, t1878: F, t774: F, t2642: F, t3280: F, t3284: F, t3348: F, t3349: F, t5517: F, t5641: F, t7093: F, t7096: F, t7100: F, t7101: F, t7108: F, t7112: F, t7115: F) -> (F, F, F, F, F) {
    let t7168 = t2630 * t443;
    let t7178 = t2634 * t450;
    let t7181 = t774 * t1878;
    let t7185 = t2642 * t450;
    let t7190 = -t7093 + t5517 + t7096 - F::new(3.44851) * t5641 + t7100 - t7101 - F::new(0.7663355555555555) * t3349 + t3280 - t3284 - t7108 + t7112 + t7115 - t3348;
    (t7168, t7178, t7181, t7185, t7190)
}
