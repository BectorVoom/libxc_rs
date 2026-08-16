//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 987/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk987<F: Float>(t12987: F, t480: F, t1224: F, t3362: F, t12268: F, t3698: F, t3367: F, t404: F, t12256: F, t11239: F, t460: F) -> (F, F, F, F, F, F) {
    let t12988 = t12987 * t480;
    let t13006 = t1224 * t3362;
    let t13020 = t3698 * t12268;
    let t13026 = F::cast_from(1.0_f64) / t404 / t3367;
    let t13027 = t13026 * t12256;
    let t13036 = t460 * t11239;
    (t12988, t13006, t13020, t13026, t13027, t13036)
}
