//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1148/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1148<F: Float>(t1253: F, t7584: F, t36093: F, t6213: F, t10248: F, t153681: F, t153684: F, t153687: F, t153689: F, t153696: F, t153698: F, t2665: F, t28987: F, t33808: F, t34312: F, t36057: F, t4135: F, t6210: F, t6216: F, t684: F, t6972: F, t7684: F) -> F {
    let t153705 = t7584 * t1253;
    let t153710 = t36093 * t6213;
    let t153712 = -t34312 * t6972 / F::new(3.0) + t153681 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t153684 - t4135 * t7684 - F::new(12.0) * t153687 - t6216 * t2665 * t153689 * t684 / F::new(9.0) - t33808 * t28987 / F::new(18.0) + t153696 / F::new(54.0) - t6216 * t2665 * t153698 * t684 / F::new(9.0) - t6210 * t36057 / F::new(3.0) + t6216 * t10248 * t153705 * t684 / F::new(9.0) - t153710 / F::new(18.0);
    t153712
}
