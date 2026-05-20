//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2224/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2224<F: Float>(t27479: F, t3215: F, t100315: F, t100321: F, t100324: F, t100327: F, t100329: F, t100332: F, t100334: F, t1028: F, t15606: F, t15975: F, t27498: F, t27528: F, t27532: F, t3208: F, t93548: F, t93813: F) -> F {
    let t100336 = F::cast_from(0.57165357490759649296e-3_f64) * t27479 * t3215;
    let t100337 = -F::cast_from(0.28582678745379824648e-3_f64) * t27498 * t15975 + F::cast_from(0.85748036236139473944e-3_f64) * t93548 * t15606 + t100315 * t27528 / F::new(27.0) - F::new(2.0) / F::new(81.0) * t100315 * t27532 - t93813 / F::new(432.0) + F::cast_from(0.85748036236139473944e-3_f64) * t100321 * t3208 + F::cast_from(0.45732285992607719436e-2_f64) * t100324 * t1028 + F::cast_from(0.30488190661738479624e-2_f64) * t100327 + F::cast_from(0.95275595817932748827e-4_f64) * t100329 - t100332 - t100334 - t100336;
    t100337
}
