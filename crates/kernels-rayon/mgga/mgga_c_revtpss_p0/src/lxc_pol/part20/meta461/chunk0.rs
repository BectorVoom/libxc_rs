//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1753/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753(t3869: f64, t39532: f64, t123: f64, t2630: f64, t3850: f64, t9575: f64, t9860: f64, t39538: f64, t39427: f64, t39535: f64, t187: f64, t47055: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47131 = 0.21687162600603479684e-1_f64 * t3869 * t39532;
    let t47133 = t3850 * t123 * t2630;
    let t47134 = 0.65061487801810439052e-1_f64 * t47133;
    let t47135 = t9860 * t9575;
    let t47136 = 0.86748650402413918736e-1_f64 * t47135;
    let t47138 = 0.43374325201206959368e-1_f64 * t3869 * t39538;
    let t47140 = 0.12842595503380418954e1_f64 * t3869 * t39427;
    let t47142 = 0.38025319932552508021e2_f64 * t3869 * t39535;
    let t47144 = 0.19751673498613801407e-1_f64 * t47055 * t187;
    (t47131, t47134, t47136, t47138, t47140, t47142, t47144)
}
