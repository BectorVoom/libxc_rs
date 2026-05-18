//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1282/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1282<F: Float>(t11675: F, t24271: F, t10349: F, t11694: F, t332: F, t3225: F, t10153: F, t35751: F, t6182: F, t11683: F, t11687: F, t22442: F) -> (F, F, F, F, F) {
    let t35831 = t11675 * t24271;
    let t35834 = t11694 * t332 * t10349;
    let t35835 = t3225 * t35834;
    let t35838 = t10153 * t35751 * t6182;
    let t35841 = t11687 * t11683 * t22442;
    (t35831, t35834, t35835, t35838, t35841)
}
