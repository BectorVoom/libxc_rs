//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1990/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1990<F: Float>(t102528: F, t102530: F, t102531: F, t102534: F, t102535: F, t102537: F, t102548: F, t108590: F, t108592: F, t94498: F, t96326: F, t98224: F, t98260: F) -> F {
    let t109822 = -F::cast_from(0.80031500487063509015e-2_f64) * t108590 + F::cast_from(0.40015750243531754507e-2_f64) * t108592 - t102528 - F::cast_from(0.45351183609335988441e-1_f64) * t98224 + t102530 - t102531 - t102534 + t102535 + t96326 + t102537 + F::cast_from(0.54208002996571016773e-3_f64) * t94498 - t102548 - F::new(35.0) / F::new(54.0) * t98260;
    t109822
}
