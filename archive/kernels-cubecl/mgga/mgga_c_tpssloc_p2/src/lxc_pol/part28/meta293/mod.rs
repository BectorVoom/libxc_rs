//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta293<F: Float>(t10191: F, t2986: F, t271: F, t2775: F, t974: F, t2769: F, t632: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F) -> (F, F, F, F, F, F, F) {
        let (t10192, t10213, t10214, t10216, t10224, t10226, t10228) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1203::<F>(t10191, t2986, t271, t2775, t974, t2769, t632, t698, t976, t979, t973, t2970, t2999);
    (t10192, t10213, t10214, t10216, t10224, t10226, t10228)
}
