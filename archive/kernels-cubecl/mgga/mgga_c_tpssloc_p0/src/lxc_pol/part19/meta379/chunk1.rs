//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1417/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417<F: Float>(t43936: F, t43949: F, t449: F, t300: F, t1098: F, t11470: F, t1119: F, t11180: F, t3308: F, t3256: F, t3312: F, t3316: F) -> (F, F, F, F, F) {
    let t43951 = (t43936 + t43949) * t449;
    let t43953 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t43951;
    let t43954 = t11470 * t1098;
    let t43956 = F::cast_from(4.0_f64) * t43954 * t1119;
    let t43958 = F::cast_from(6.0_f64) * t11180 * t3308;
    let t43959 = t3256 * t3312;
    let t43961 = F::cast_from(0.96491876992155210402e2_f64) * t43959 * t3316;
    (t43951, t43953, t43956, t43958, t43961)
}
