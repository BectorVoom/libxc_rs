//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta613<F: Float>(t10383: F, t1926: F, t3014: F, t40: F, t1933: F, t23479: F, t1004: F, t23528: F, t23544: F, t3053: F, t10948: F, t23536: F) -> (F, F, F, F, F, F) {
        let (t83028, t83032, t83034, t83038, t83041, t83043) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2088::<F>(t10383, t1926, t3014, t40, t1933, t23479, t1004, t23528, t23544, t3053, t10948, t23536);
    (t83028, t83032, t83034, t83038, t83041, t83043)
}
