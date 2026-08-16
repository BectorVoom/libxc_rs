//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta247<F: Float>(t3448: F, t6138: F, t6144: F, t11583: F, t5392: F, t15338: F, t4904: F, t3447: F, t3431: F, t6126: F, t1174: F, t6130: F) -> (F, F, F, F, F, F, F, F) {
        let (t18416, t18420, t18427, t18446, t18447, t18451, t18452, t18454) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk906::<F>(t3448, t6138, t6144, t11583, t5392, t15338, t4904, t3447, t3431, t6126, t1174, t6130);
    (t18416, t18420, t18427, t18446, t18447, t18451, t18452, t18454)
}
