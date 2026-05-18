//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 764/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk764<F: Float>(t32385: F, t5507: F, t28: F, t1286: F, t1310: F, t31997: F, t32000: F, t32002: F, t32013: F, t32016: F, t32021: F, t32025: F, t32054: F, t32366: F, t32371: F, t32375: F, t32380: F, t5495: F, t5501: F, t5504: F, t5620: F, t5624: F, t7162: F, t7168: F, t7214: F, t7218: F) -> (F, F, F) {
    let t32386 = t5507 * t32385;
    let t32387 = t28 * t32386;
    let t32390 = -t31997 - t32000 - t1286 * t32002 / F::new(3.0) + t5495 * t7214 / F::new(6.0) + t7162 * t5624 / F::new(6.0) + t7162 * t5620 / F::new(6.0) - t5501 * t32013 / F::new(18.0) - t32016 * t5504 / F::new(18.0) + t5501 * t32021 / F::new(9.0) - t32025 + t32054 * t1310 / F::new(6.0) + F::new(2.0) * t32366 + t5495 * t7218 / F::new(3.0) + t1286 * t32371 / F::new(3.0) + t1286 * t32375 / F::new(6.0) + t1286 * t32380 / F::new(6.0) - t5495 * t7168 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1286 * t32387;
    (t32386, t32387, t32390)
}
