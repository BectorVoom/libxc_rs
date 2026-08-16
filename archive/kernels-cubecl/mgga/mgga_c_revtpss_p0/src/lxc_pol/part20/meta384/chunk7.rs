//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1409/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1409<F: Float>(t2408: F, t2410: F, t2832: F, t775: F, t10818: F, t11071: F, t198: F, t207: F, t2393: F, t2403: F, t2404: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t39857: F, t39859: F, t39861: F, t40084: F, t40088: F, t40240: F, t4541: F) -> F {
    let t41151 = t2408 * t2408;
    let t41153 = t2410 * t2410;
    let t41154 = F::cast_from(1.0_f64) / t41153;
    let t41161 = t775 * t2832;
    let t41168 = -F::cast_from(6.0_f64) * t198 * t207 * t41151 * t41154 + F::cast_from(72.0_f64) * t10818 * t2404 * t4541 - F::cast_from(36.0_f64) * t11071 * t2403 * t41161 + F::cast_from(18.0_f64) * t198 * t2393 * t40240 + t39799 + t39807 - t39813 - t39818 - t39823 + t39857 + t39859 - t39861 + t40084 + t40088;
    t41168
}
