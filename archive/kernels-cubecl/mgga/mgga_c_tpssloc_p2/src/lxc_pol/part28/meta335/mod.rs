//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta335 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta335<F: Float>(t10027: F, t541: F, t12267: F, t1362: F, t3777: F, t3865: F, t1369: F, t1361: F, t2690: F, t1336: F, t241: F, t67: F, t6924: F) -> (F, F, F, F, F, F, F) {
        let (t12335, t12336, t12339, t12340, t12345, t12346, t12351) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1268::<F>(t10027, t541, t12267, t1362, t3777, t3865, t1369, t1361, t2690, t1336, t241, t67, t6924);
    (t12335, t12336, t12339, t12340, t12345, t12346, t12351)
}
