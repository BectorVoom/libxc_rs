//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 978/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk978<F: Float>(t12045: F, t12052: F, t12054: F, t5151: F, t750: F, t17: F, t1787: F, t2516: F, t12120: F, t2663: F, t5157: F, t1788: F, t2225: F) -> (F, F, F, F, F, F, F, F) {
    let t15911 = F::cast_from(48.0_f64) * t12045;
    let t15916 = F::cast_from(12.0_f64) * t12052;
    let t15917 = F::cast_from(80.0_f64) * t12054;
    let t15921 = t5151 * t750;
    let t15923 = F::cast_from(2.0_f64) * t17 * t15921;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15976 = F::cast_from(4.0_f64) * t12120;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    (t15911, t15916, t15917, t15923, t15972, t15976, t15979, t15982)
}
