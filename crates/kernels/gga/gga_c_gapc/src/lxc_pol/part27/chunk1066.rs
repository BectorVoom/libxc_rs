//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1066/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1066<F: Float>(t10072: F, t11930: F, t11302: F, t7294: F, t8135: F, t10069: F, t15644: F, t8142: F, t1734: F, t8654: F, t2660: F, t7880: F) -> (F, F, F, F, F, F, F) {
    let t33209 = t11930 * t10072;
    let t33211 = t7294 * t11302;
    let t33212 = t33211 * t8135;
    let t33214 = t11930 * t10069;
    let t33217 = t15644 * t11302 * t8142;
    let t33219 = t1734 * t8654;
    let t33221 = t2660 * t33219 * t7880;
    (t33209, t33211, t33212, t33214, t33217, t33219, t33221)
}
