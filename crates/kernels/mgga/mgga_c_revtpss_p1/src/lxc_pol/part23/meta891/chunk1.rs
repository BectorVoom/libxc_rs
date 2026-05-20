//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2842/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2842<F: Float>(t231: F, t23244: F, t243: F, t2661: F, t2662: F, t10871: F, t40693: F, t76569: F, t23263: F, t40864: F, t23114: F, t40462: F, t40810: F, t51042: F, t51055: F, t62108: F, t62111: F, t62114: F, t62129: F, t62135: F, t62148: F, t76804: F, t76808: F, t76812: F, t76814: F, t76818: F, t775: F, t828: F, t851: F) -> F {
    let t76823 = t2661 * t2662 * t243 * t23244 * t231;
    let t76827 = t2661 * t40693 * t76569 * t10871;
    let t76835 = t40864 * t23263;
    let t76843 = F::cast_from(0.24009450146119052705e-1_f64) * t62108 + F::cast_from(0.12004725073059526352e0_f64) * t76804 - F::cast_from(0.15246000842785598467e-2_f64) * t76808 - F::cast_from(0.42874018118069736973e-3_f64) * t76812 + F::cast_from(0.40015750243531754507e-2_f64) * t76814 + F::cast_from(0.71456696863449561619e-5_f64) * t76818 + F::cast_from(0.71456696863449561619e-5_f64) * t76823 + F::cast_from(0.42874018118069736973e-4_f64) * t76827 + F::cast_from(0.18007087609589289528e0_f64) * t851 * t40462 * t828 * t23114 * t775 + F::cast_from(0.1084295579938911763e-3_f64) * t62111 + F::new(7.0) / F::new(12.0) * t76835 + F::cast_from(0.18007087609589289529e-1_f64) * t62114 + t40810 - F::cast_from(0.1372140075850703862e-3_f64) * t51042 + F::cast_from(0.45732285992607719437e-2_f64) * t62129 + F::new(7.0) / F::new(48.0) * t62135 - F::cast_from(0.38115002106963996168e-4_f64) * t62148 - F::cast_from(0.5421477899694558815e-4_f64) * t51055;
    t76843
}
