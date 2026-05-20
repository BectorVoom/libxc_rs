//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2937/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2937<F: Float>(t10356: F, t1042: F, t1063: F, t11704: F, t11994: F, t15938: F, t15952: F, t16199: F, t1671: F, t3091: F, t3092: F, t3106: F, t42193: F, t42204: F, t42584: F, t4781: F, t53427: F, t53432: F, t53433: F, t53437: F, t53450: F) -> F {
    let t53455 = -F::cast_from(0.28582678745379824648e-3_f64) * t42193 + F::cast_from(0.14481890564325777821e-1_f64) * t53427 - t53432 + F::cast_from(0.57165357490759649295e-3_f64) * t53433 + F::cast_from(0.30488190661738479624e-2_f64) * t42204 - F::cast_from(0.95275595817932748825e-4_f64) * t53437 + F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t4781 * t11704 * t10356 - F::cast_from(0.13719685797782315831e-1_f64) * t3106 * t15938 - F::cast_from(0.53100265402527852012e-1_f64) * t42584 * t1671 - F::cast_from(0.85748036236139473944e-3_f64) * t11994 * t15952 - F::cast_from(0.42874018118069736973e-2_f64) * t1063 * t1042 * t16199 * t53450;
    t53455
}
