//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1261/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1261<F: Float>(t102397: F, t121076: F, t121178: F, t122346: F, t122413: F, t125749: F, t125753: F, t125767: F, t14230: F, t2022: F, t2097: F, t27864: F, t27960: F, t28888: F, t32686: F, t32719: F, t34231: F, t7274: F, t8085: F, t8706: F, t8707: F) -> F {
    let t128742 = F::cast_from(0.34271842599061411569e1_f64) * t32719 * t102397 * t2022 * t14230 - F::cast_from(0.22312397525430606492e-2_f64) * t125749 - F::cast_from(0.14874931683620404328e-2_f64) * t125753 + F::cast_from(0.57119737665102352616e0_f64) * t8706 * t8707 * t28888 * t2022 + F::cast_from(0.57119737665102352616e0_f64) * t8706 * t8707 * t8085 * t7274 - t122413 - F::cast_from(0.17135921299530705785e1_f64) * t34231 * t32686 + F::cast_from(0.57119737665102352616e0_f64) * t8706 * t8707 * t2097 * t27960 + F::cast_from(0.34271842599061411569e1_f64) * t121076 * t122346 * t27864 - F::cast_from(0.66934509195437693771e-4_f64) * t121178 - F::cast_from(0.34708173928447610099e-2_f64) * t125767;
    t128742
}
