//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta353(t3259: f64, t359: f64, t11239: f64, t3143: f64, t342: f64, t1086: f64, t3043: f64, t3298: f64, t989: f64, t4980: f64, t994: f64, t4995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12073, t12077, t12078, t12097, t12116, t12122, t12127) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1373(t3259, t359, t11239, t3143, t342, t1086, t3043, t3298, t989, t4980, t994, t4995);
    (t12073, t12077, t12078, t12097, t12116, t12122, t12127)
}
