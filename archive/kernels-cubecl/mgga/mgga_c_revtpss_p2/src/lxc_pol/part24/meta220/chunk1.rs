//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 970/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970<F: Float>(t11940: F, t366: F, t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F, F, F) {
    let t11941 = t11940 * t366;
    let t11970 = t371 * t2434 * t373;
    let t11972 = F::cast_from(0.63517063878621832551e-4_f64) * t367 * t11970;
    let t11986 = t675 * t1065;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = F::cast_from(1.0_f64) / t3145 / t334;
    (t11941, t11970, t11972, t11986, t12046, t12047, t12050)
}
