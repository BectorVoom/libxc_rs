//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk822;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta152(t422: f64, t3265: f64, t3313: f64, t3236: f64, t3238: f64, t3245: f64, t3250: f64, t3254: f64, t1124: f64, t1128: f64, t1127: f64, t432: f64, t427: f64, t1136: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3314, t3315, t3316, t3318, t3319, t3324, t3327, t3330) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk822(t422, t3265, t3313, t3236, t3238, t3245, t3250, t3254, t1124, t1128, t1127, t432);
        let (t3331, t3332, t3333) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk823(t3330, t427, t1136);
    (t3314, t3315, t3316, t3318, t3319, t3324, t3327, t3331, t3332, t3333)
}
