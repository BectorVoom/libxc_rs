//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta417<F: Float>(t3361: F, t39443: F, t141: F, t43764: F, t1146: F, t9303: F, t12270: F, t698: F, t2304: F, t12254: F, t2439: F, t3424: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t43766, t43767, t43769, t43771, t43773, t43776, t43777, t43779, t43781) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1559::<F>(t3361, t39443, t141, t43764, t1146, t9303, t12270, t698, t2304, t12254, t2439, t3424);
    (t43766, t43767, t43769, t43771, t43773, t43776, t43777, t43779, t43781)
}
