//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1241/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1241<F: Float>(t10356: F, t13020: F, t1012: F, t3367: F, t404: F, t12256: F, t1204: F, t3140: F, t3599: F, t11239: F, t460: F, t1242: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13021 = t13020 * t10356;
    let t13022 = t1012 * t13021;
    let t13026 = F::cast_from(1.0_f64) / t404 / t3367;
    let t13027 = t13026 * t12256;
    let t13028 = t13027 * t10356;
    let t13029 = t1012 * t13028;
    let t13032 = t1204 * t3140;
    let t13033 = t13032 * t3599;
    let t13036 = t460 * t11239;
    let t13037 = t1242 * t1242;
    (t13021, t13022, t13026, t13028, t13029, t13032, t13033, t13036, t13037)
}
