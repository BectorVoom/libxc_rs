//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2149/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2149<F: Float>(t6317: F, t7131: F, t100025: F, t100055: F, t100160: F, t100166: F, t100230: F, t1068: F, t15670: F, t1675: F, t19745: F, t19864: F, t19986: F, t20046: F, t25580: F, t27489: F, t4831: F, t4839: F, t4907: F, t7132: F, t93752: F) -> F {
    let t106971 = t6317 * t7131;
    let t106990 = F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t20046 + F::cast_from(0.28582678745379824648e-3_f64) * t106971 * t1068 + F::cast_from(0.17149607247227894789e-2_f64) * t15670 * t7131 * t4839 - F::cast_from(0.57165357490759649296e-3_f64) * t100055 * t19986 - F::cast_from(0.85748036236139473944e-3_f64) * t100025 * t4907 - F::cast_from(0.42874018118069736972e-3_f64) * t25580 * t19745 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t19864 - t100160 - F::cast_from(0.38110238327173099531e-3_f64) * t100166 + F::cast_from(0.57165357490759649296e-3_f64) * t100230 * t1675 + F::cast_from(0.57165357490759649296e-3_f64) * t27489 * t4831;
    t106990
}
