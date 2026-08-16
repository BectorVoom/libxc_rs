//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1753/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1753<F: Float>(t3869: F, t39532: F, t123: F, t2630: F, t3850: F, t9575: F, t9860: F, t39538: F, t39427: F, t39535: F, t187: F, t47055: F) -> (F, F, F, F, F, F, F) {
    let t47131 = F::cast_from(0.21687162600603479684e-1_f64) * t3869 * t39532;
    let t47133 = t3850 * t123 * t2630;
    let t47134 = F::cast_from(0.65061487801810439052e-1_f64) * t47133;
    let t47135 = t9860 * t9575;
    let t47136 = F::cast_from(0.86748650402413918736e-1_f64) * t47135;
    let t47138 = F::cast_from(0.43374325201206959368e-1_f64) * t3869 * t39538;
    let t47140 = F::cast_from(0.12842595503380418954e1_f64) * t3869 * t39427;
    let t47142 = F::cast_from(0.38025319932552508021e2_f64) * t3869 * t39535;
    let t47144 = F::cast_from(0.19751673498613801407e-1_f64) * t47055 * t187;
    (t47131, t47134, t47136, t47138, t47140, t47142, t47144)
}
