//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2715/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715<F: Float>(t49966: F, t10600: F, t18259: F, t14325: F, t14390: F, t14468: F, t1544: F, t2403: F, t2404: F, t39783: F, t41197: F, t49950: F, t49956: F, t49958: F, t49959: F, t49964: F, t775: F) -> (F, F, F, F) {
    let t49967 = F::cast_from(0.17544670867903938621e1_f64) * t49966;
    let t49969 = F::new(36.0) * t18259 * t10600;
    let t49971 = F::new(72.0) * t14325 * t14390;
    let t49972 = F::new(9.0) * t14468 * t2403 * t2404 + F::new(3.0) * t1544 * t2403 * t41197 + F::new(9.0) * t2403 * t49950 * t775 - t39783 + t49956 - t49958 + t49959 - t49964 - t49967 + t49969 + t49971;
    (t49967, t49969, t49971, t49972)
}
