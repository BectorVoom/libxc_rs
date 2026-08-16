//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1793/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1793<F: Float>(t25224: F, t6572: F, t1880: F, t13053: F, t1528: F, t1912: F, t23235: F, t23281: F, t25200: F, t25206: F, t25209: F, t25211: F, t25214: F, t25218: F, t25220: F, t25222: F, t259: F, t2713: F, t7538: F, t855: F) -> (F, F) {
    let t25225 = t25224 * t6572;
    let t25226 = t1880 * t25225;
    let t25228 = F::cast_from(0.19190897446562641759e-1_f64) * t23235 + F::cast_from(2.0_f64) * t855 * t25200 - t2713 * t7538 - t23281 * t1528 + F::cast_from(0.41123351671205660912e-2_f64) * t25206 - t13053 * t1912 + F::cast_from(0.38381794893125283518e-1_f64) * t25209 + F::cast_from(0.19190897446562641759e-1_f64) * t25211 - F::cast_from(0.82246703342411321825e-2_f64) * t25214 - F::cast_from(0.82246703342411321825e-2_f64) * t25218 + t25220 * t259 + t25222 * t259 - F::cast_from(0.82246703342411321825e-2_f64) * t25226;
    (t25225, t25228)
}
