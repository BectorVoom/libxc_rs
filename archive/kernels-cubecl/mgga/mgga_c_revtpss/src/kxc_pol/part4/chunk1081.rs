//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1081/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1081<F: Float>(t1222: F, t13011: F, t140: F, t3688: F, t3700: F, t3367: F, t404: F, t1242: F, t3603: F, t471: F, t1032: F, t3552: F) -> (F, F, F, F, F, F, F) {
    let t13012 = t1222 * t13011;
    let t13014 = t140 * t3688;
    let t13015 = t1222 * t13014;
    let t13017 = t140 * t3700;
    let t13018 = t1222 * t13017;
    let t13026 = F::cast_from(1.0_f64) / t404 / t3367;
    let t13037 = t1242 * t1242;
    let t13038 = F::cast_from(1.0_f64) / t13037;
    let t13045 = t3603 * t471;
    let t13068 = t3552 * t1032;
    (t13012, t13015, t13018, t13026, t13038, t13045, t13068)
}
