//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 730/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk730<F: Float>(t1243: F, t4620: F, t1953: F, t1966: F, t945: F, t11: F, t940: F, t503: F, t1251: F, t34: F, t348: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4621 = t1243 * t4620;
    let t4622 = t1953 * t4621;
    let t4624 = t1966 * t945;
    let t4625 = t1243 * t4624;
    let t4626 = t11 * t4625;
    let t4628 = t1966 * t940;
    let t4629 = t503 * t4628;
    let t4630 = t11 * t4629;
    let t4632 = t1251 * t34;
    let t4633 = t4632 * t348;
    let t4634 = t503 * t4633;
    let t4635 = t1953 * t4634;
    (t4621, t4622, t4624, t4625, t4626, t4628, t4629, t4630, t4632, t4633, t4634, t4635)
}
