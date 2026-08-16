//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1358;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta299<F: Float>(t10294: F, t2403: F, t909: F, t2827: F, t699: F, t2830: F, t2833: F, t241: F, t2978: F, t2955: F, t969: F, t2967: F, t964: F) -> (F, F, F, F, F, F, F, F) {
        let (t10295, t10296, t10298, t10300, t10302, t10304, t10331, t10333) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1358::<F>(t10294, t2403, t909, t2827, t699, t2830, t2833, t241, t2978, t2955, t969, t2967, t964);
    (t10295, t10296, t10298, t10300, t10302, t10304, t10331, t10333)
}
