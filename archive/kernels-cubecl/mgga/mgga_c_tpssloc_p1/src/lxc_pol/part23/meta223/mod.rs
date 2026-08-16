//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta223<F: Float>(t1229: F, t3242: F, t11153: F, t3584: F, t1734: F, t3508: F, t1089: F, t475: F, t1744: F, t3540: F, t1731: F, t1706: F, t3545: F) -> (F, F, F, F, F, F, F) {
        let (t15615, t15654, t15659, t15701, t15717, t15719, t15727) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk871::<F>(t1229, t3242, t11153, t3584, t1734, t3508, t1089, t475, t1744, t3540, t1731, t1706, t3545);
    (t15615, t15654, t15659, t15701, t15717, t15719, t15727)
}
