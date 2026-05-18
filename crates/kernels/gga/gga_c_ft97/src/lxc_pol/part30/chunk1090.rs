//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1090/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1090<F: Float>(t25462: F, t35814: F, t10248: F, t10683: F, t1091: F, t142485: F, t142663: F, t142913: F, t25459: F, t2665: F, t28501: F, t29000: F, t33996: F, t34001: F, t34006: F, t36109: F, t3746: F, t4162: F, t44280: F, t6216: F, t6217: F, t6967: F) -> F {
    let t152590 = t25462 * t35814;
    let t152615 = -F::new(4.0) * t6216 * t44280 * t34006 * t4162 + F::new(2.0) * t6216 * t10683 * t6217 * t28501 - t25459 * t36109 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t29000 * t2665 * t33996 * t3746 - t152590 / F::new(27.0) + t6216 * t10683 * t34001 * t4162 + t6216 * t10248 * t142913 * t1091 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t29000 * t10248 * t34006 * t3746 - t142663 * t6967 / F::new(18.0) - t6216 * t2665 * t142485 * t1091 / F::new(18.0) + t29000 * t2665 * t34001 * t3746 / F::new(9.0) + t25459 * t35814 / F::new(9.0);
    t152615
}
