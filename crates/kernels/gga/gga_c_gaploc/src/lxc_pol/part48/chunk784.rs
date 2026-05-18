//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 784/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk784<F: Float>(t2476: F, t26922: F, t9438: F, t10268: F, t4391: F, t549: F, t2365: F, t31748: F, t12996: F, t18067: F, t31586: F, t31591: F) -> (F, F, F, F, F, F) {
    let t41615 = t2476 * t9438 * t26922;
    let t41618 = t4391 * t549 * t10268;
    let t41621 = t4391 * t2365 * t31748;
    let t41623 = t18067 * t12996;
    let t41626 = t4391 * t2365 * t31586;
    let t41629 = t4391 * t2365 * t31591;
    (t41615, t41618, t41621, t41623, t41626, t41629)
}
