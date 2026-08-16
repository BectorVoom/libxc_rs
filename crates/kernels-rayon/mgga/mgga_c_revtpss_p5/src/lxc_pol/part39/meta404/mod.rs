//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta404(t3: f64, t31087: f64, t2178: f64, t2327: f64, t116: f64, t8273: f64, t670: f64, t2371: f64, t8295: f64, t117: f64, t31066: f64, t1459: f64, t1461: f64, t2187: f64, t2189: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t8289: f64, t8296: f64, t8299: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31088, t31100, t31114, t31117, t31118, t31121, t31124, t31127) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1478(t3, t31087, t2178, t2327, t116, t8273, t670, t2371, t8295, t117, t31066, t1459, t1461, t2187, t2189, t4158, t4162, t4165, t572, t573, t8289, t8296, t8299, param_d);
    (t31088, t31100, t31114, t31117, t31118, t31121, t31124, t31127)
}
