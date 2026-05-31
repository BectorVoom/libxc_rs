//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 845/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk845<F: Float>(t530: F, t7792: F, t186: F, t185: F, t7359: F, t198: F, t493: F, t6280: F, t739: F, t1313: F, t519: F, t2393: F, t4738: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7793 = t530 * t7792;
    let t7794 = t186 * t7793;
    let t7796 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t185 * t7794;
    let t7797 = F::cast_from(3.0_f64) * t7359;
    let t7798 = t198 * t7797;
    let t7799 = t186 * t7798;
    let t7801 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t493 * t7799;
    let t7802 = t6280 * t739;
    let t7803 = t1313 * t7802;
    let t7805 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t7803;
    let t7807 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t4738 * t2393;
    (t7793, t7794, t7796, t7797, t7798, t7799, t7801, t7802, t7803, t7805, t7807)
}
