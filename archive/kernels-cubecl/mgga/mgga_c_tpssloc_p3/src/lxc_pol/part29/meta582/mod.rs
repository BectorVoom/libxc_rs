//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta582<F: Float>(t154: F, t8705: F, t1887: F, t534: F, t12267: F, t6951: F, t131: F, t22791: F, t9537: F, t1338: F, t225: F, t236: F) -> (F, F, F, F, F, F) {
        let (t80845, t80848, t80849, t80853, t80854, t80855) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2002::<F>(t154, t8705, t1887, t534, t12267, t6951, t131, t22791, t9537, t1338, t225, t236);
    (t80845, t80848, t80849, t80853, t80854, t80855)
}
