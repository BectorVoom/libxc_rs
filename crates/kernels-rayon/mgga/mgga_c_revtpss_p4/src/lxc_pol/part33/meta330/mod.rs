//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta330(t11239: f64, t3143: f64, t342: f64, t3298: f64, t989: f64, t4980: f64, t994: f64, t4995: f64, t1043: f64, t3153: f64, t3046: f64, t3286: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12077, t12078, t12116, t12122, t12127, t12131, t12146) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1340(t11239, t3143, t342, t3298, t989, t4980, t994, t4995, t1043, t3153, t3046, t3286);
    (t12077, t12078, t12116, t12122, t12127, t12131, t12146)
}
