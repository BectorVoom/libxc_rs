//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 663/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk663<F: Float>(t22623: F, t8502: F, t2021: F, t8774: F, t10007: F, t8669: F, t197: F, t2754: F, t1: F, t20550: F, t7892: F, t7905: F, t9448: F, t9439: F, t1415: F, t8247: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25070 = t22623 * t8502;
    let t25198 = t2021 * t8774;
    let t25359 = t10007 * t8669;
    let t25760 = t197 * t2754;
    let t26126 = t25760 * t1;
    let t26328 = t20550 * t7892;
    let t26435 = t9448 * t7905;
    let t26922 = t9439 * t7905;
    let t26984 = t1415 * t8247;
    (t25070, t25198, t25359, t25760, t26126, t26328, t26435, t26922, t26984)
}
