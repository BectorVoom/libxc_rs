//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta506<F: Float>(t15957: F, t3095: F, t3092: F, t2857: F, t357: F, t2251: F, t4781: F, t11659: F, t3154: F, t1592: F, t11710: F, t4782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15958, t15959, t15963, t15964, t15965, t15968, t15969, t15970, t15973, t15974, t15975, t15984) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2125::<F>(t15957, t3095, t3092, t2857, t357, t2251, t4781, t11659, t3154, t1592, t11710, t4782);
    (t15958, t15959, t15963, t15964, t15965, t15968, t15969, t15970, t15973, t15974, t15975, t15984)
}
