//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2234/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2234<F: Float>(t26827: F, t5362: F, t17435: F, t7613: F, t3670: F, t8184: F, t12702: F, t12744: F, t17391: F, t17602: F, t17744: F, t26870: F, t29062: F, t29096: F, t3663: F, t3674: F, t5335: F, t5343: F, t5348: F, t97182: F, t97191: F) -> F {
    let t104815 = F::cast_from(0.57165357490759649296e-3_f64) * t26827 * t5362;
    let t104817 = F::cast_from(0.57165357490759649296e-3_f64) * t7613 * t17435;
    let t104818 = t3670 * t8184;
    let t104821 = -F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t17744 + F::cast_from(0.17149607247227894789e-2_f64) * t12702 * t29096 * t5343 - F::cast_from(0.85748036236139473944e-3_f64) * t12744 * t29096 * t5335 - F::cast_from(0.85748036236139473944e-3_f64) * t97182 * t5348 - F::cast_from(0.85748036236139473944e-3_f64) * t26870 * t17391 - F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t17602 - F::cast_from(0.57165357490759649296e-3_f64) * t97191 + F::cast_from(0.22866142996303859718e-2_f64) * t29062 * t3663 - t104815 - t104817 - F::cast_from(0.45732285992607719436e-2_f64) * t104818 * t3674;
    t104821
}
