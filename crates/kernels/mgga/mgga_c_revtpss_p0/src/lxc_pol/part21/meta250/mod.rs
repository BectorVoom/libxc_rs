//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta250 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta250<F: Float>(t9484: F, t9543: F, t520: F, t512: F, t1450: F, t4135: F, t177: F, t3850: F, t762: F, t749: F, t1353: F, t198: F, t4139: F, t566: F, t9399: F, t9400: F, t9405: F, t9407: F, t9409: F, t9412: F, t9415: F, t9421: F, t9423: F, t9427: F, t9430: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9544, t9545, t9546, t9547, t9551, t9552, t9553, t9554, t9555, t9556, t9557) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1436::<F>(t9484, t9543, t520, t512, t1450, t4135, t177, t3850, t762, t749, t1353, t198, t4139, t566, t9399, t9400, t9405, t9407, t9409, t9412, t9415, t9421, t9423, t9427, t9430);
    (t9544, t9545, t9546, t9547, t9551, t9552, t9553, t9554, t9555, t9556, t9557)
}
