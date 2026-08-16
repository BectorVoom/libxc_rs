//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta257<F: Float>(t592: F, t6328: F, t3701: F, t6463: F, t11987: F, t6305: F, t12000: F, t6312: F, t1814: F, t5333: F, t1338: F, t6434: F) -> (F, F, F, F, F, F) {
        let (t19593, t19596, t19606, t19618, t19654, t19657) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk918::<F>(t592, t6328, t3701, t6463, t11987, t6305, t12000, t6312, t1814, t5333, t1338, t6434);
    (t19593, t19596, t19606, t19618, t19654, t19657)
}
