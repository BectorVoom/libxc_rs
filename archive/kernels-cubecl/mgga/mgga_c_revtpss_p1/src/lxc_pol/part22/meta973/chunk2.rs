//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3262/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3262<F: Float>(t10777: F, t18481: F, t50945: F, t18333: F, t51123: F, t18349: F, t2689: F, t14494: F, t14785: F, t14791: F, t14894: F, t2745: F, t36833: F, t40732: F, t4424: F, t4433: F, t50423: F, t50474: F, t50722: F, t50724: F, t50728: F, t50732: F) -> F {
    let t61913 = t10777 * t50945 * t18481;
    let t61916 = t10777 * t51123 * t18333;
    let t61924 = t2689 * t18349;
    let t61929 = F::cast_from(0.10289764348336736873e-1_f64) * t14894 * t14791 * t50474 * t50423 - F::cast_from(0.17149607247227894789e-1_f64) * t2745 * t14785 * t4424 * t4433 - F::cast_from(0.10164000561857065645e-2_f64) * t61913 + F::cast_from(0.2032800112371413129e-3_f64) * t61916 - F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t36833 * t14494 * t4424 - F::cast_from(0.10841600599314203354e-2_f64) * t40732 - F::cast_from(0.16006300097412701803e-1_f64) * t50722 - F::cast_from(0.30488190661738479625e-3_f64) * t61924 + F::cast_from(0.24009450146119052705e0_f64) * t50724 - F::cast_from(0.57165357490759649296e-4_f64) * t50728 + F::cast_from(0.28582678745379824648e-4_f64) * t50732;
    t61929
}
