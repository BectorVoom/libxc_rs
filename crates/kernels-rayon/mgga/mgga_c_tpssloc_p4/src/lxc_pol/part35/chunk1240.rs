//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1240/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1240(t1761: f64, t19232: f64, t19234: f64, t19249: f64, t2155: f64, t24587: f64, t27401: f64, t27406: f64, t27830: f64, t29667: f64, t29699: f64, t29825: f64, t4945: f64, t8006: f64, t8015: f64, t8061: f64, t8088: f64) -> f64 {
    let t29827 = 4.0_f64 * t4945 * t8061 - t19249 * t2155 - t24587 - t19232 * t2155 + 0.43864908449286038306e-1_f64 * t27406 * t8015 + 0.43864908449286038306e-1_f64 * t27406 * t8006 - 2.0_f64 * t19234 * t2155 - 2.0_f64 * t4945 * t8088 - 0.18277045187202515961e-2_f64 * t27401 - 2.0_f64 * t27830 * t1761 + t29667 + t29699 + t29825;
    t29827
}
