//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta404<F: Float>(t3: F, t31087: F, t2178: F, t2327: F, t116: F, t8273: F, t670: F, t2371: F, t8295: F, t117: F, t31066: F, t1459: F, t1461: F, t2187: F, t2189: F, t4158: F, t4162: F, t4165: F, t572: F, t573: F, t8289: F, t8296: F, t8299: F, param_d: F) -> (F, F, F, F, F, F, F, F) {
        let (t31088, t31100, t31114, t31117, t31118, t31121, t31124, t31127) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1478::<F>(t3, t31087, t2178, t2327, t116, t8273, t670, t2371, t8295, t117, t31066, t1459, t1461, t2187, t2189, t4158, t4162, t4165, t572, t573, t8289, t8296, t8299, param_d);
    (t31088, t31100, t31114, t31117, t31118, t31121, t31124, t31127)
}
