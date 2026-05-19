//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 864/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk864<F: Float>(t26021: F, t7262: F, t820: F, t843: F, t1401: F, t241: F, t3920: F, t7246: F, t2023: F, t2453: F, t3908: F, t72: F, t7307: F) -> (F, F, F, F, F, F, F) {
    let t26022 = F::cast_from(0.90357964994909313586e-5_f64) * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26025 = t26024 * t1401;
    let t26028 = t820 * t7262 * t241;
    let t26040 = F::cast_from(0.13009920719177044025e-1_f64) * t7246 * t3920;
    let t26041 = t2453 * t2023;
    let t26043 = F::cast_from(0.11565819519348392139e-2_f64) * t26041 * t3908;
    let t26049 = t7307 * t72;
    (t26022, t26024, t26025, t26028, t26040, t26043, t26049)
}
