//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 535/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk535<F: Float>(t309: F, t310: F, t311: F, t305: F, t296: F, t343: F, t1051: F, t156: F, t1084: F, t1055: F, t357: F, t4: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2693 = F::cast_from(1.0_f64) / t311 / t310 / t309;
    let t2694 = t305 * t2693;
    let t2695 = t343 * t296;
    let t2696 = t2694 * t2695;
    let t2698 = t156 * t1051;
    let t2699 = t1084 * t2698;
    let t2700 = F::cast_from(0.016265371324172287_f64) * t2699;
    let t2701 = t156 * t1055;
    let t2702 = t1084 * t2701;
    let t2703 = F::cast_from(0.4815944609513912_f64) * t2702;
    let t2704 = t357 * t4;
    (t2693, t2694, t2695, t2696, t2698, t2699, t2700, t2701, t2702, t2703, t2704)
}
