//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1193/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1193<F: Float>(t1036: F, t11316: F, t15341: F, t1030: F, t12768: F, t1749: F, t11438: F, t21649: F, t3021: F, t1649: F, t33303: F, t5553: F) -> (F, F, F, F, F) {
    let t34785 = t11316 * t1036 * t15341;
    let t34788 = t1030 * t12768 * t1749;
    let t34791 = t11438 * t3021 * t21649;
    let t34793 = t33303 * t1649;
    let t34794 = t5553 * t34793;
    (t34785, t34788, t34791, t34793, t34794)
}
