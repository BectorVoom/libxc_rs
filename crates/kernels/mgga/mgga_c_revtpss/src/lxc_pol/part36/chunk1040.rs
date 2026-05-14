//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1040/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1040<F: Float>(t2142: F, t3566: F, t7635: F, t12587: F, t2155: F, t1962: F, t198: F, t205: F, t30: F, t892: F, t689: F, t7774: F, t25411: F, t213: F, t7759: F, t25431: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26976 = t3566 * t2142;
    let t26994 = t3566 * t7635;
    let t27041 = t2155 * t12587;
    let t27158 = t198 * t205 * t1962;
    let t27159 = t892 * t30;
    let t27186 = t7774 * t689;
    let t27187 = t25411 * t27186;
    let t27189 = t213 * t7759;
    let t27192 = t25431 * t27186;
    (t26976, t26994, t27041, t27158, t27159, t27186, t27187, t27189, t27192)
}
