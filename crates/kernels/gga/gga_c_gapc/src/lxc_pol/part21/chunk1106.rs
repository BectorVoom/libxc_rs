//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1106/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1106<F: Float>(t11626: F, t3234: F, t6179: F, t11625: F, t11669: F, t2440: F, t3728: F, t7029: F, t11675: F, t24271: F, t10349: F, t11694: F, t332: F, t3225: F, t10153: F, t35751: F, t6182: F) -> (F, F, F, F, F, F, F) {
    let t35823 = t3234 * t6179 * t11626;
    let t35826 = t11625 * t11669 * t2440;
    let t35829 = t11625 * t3728 * t7029;
    let t35831 = t11675 * t24271;
    let t35834 = t11694 * t332 * t10349;
    let t35835 = t3225 * t35834;
    let t35838 = t10153 * t35751 * t6182;
    (t35823, t35826, t35829, t35831, t35834, t35835, t35838)
}
