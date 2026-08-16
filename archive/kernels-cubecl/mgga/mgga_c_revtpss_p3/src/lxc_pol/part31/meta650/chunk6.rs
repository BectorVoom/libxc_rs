//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2150/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2150<F: Float>(t100008: F, t100138: F, t100141: F, t100186: F, t19682: F, t19688: F, t19693: F, t19707: F, t19722: F, t19750: F, t19754: F, t19758: F, t19792: F, t25522: F, t6273: F, t7132: F, t93548: F, t93670: F, t99985: F) -> F {
    let t107012 = F::cast_from(0.42874018118069736972e-3_f64) * t99985 * t19722 + F::cast_from(0.25724410870841842183e-2_f64) * t100138 * t19750 - F::cast_from(0.25724410870841842183e-2_f64) * t100141 * t19754 + F::cast_from(0.42874018118069736972e-3_f64) * t93548 * t19758 + F::cast_from(0.45732285992607719437e-2_f64) * t93670 * t6273 - t100186 + F::cast_from(0.11433071498151929859e-2_f64) * t100008 * t19707 - F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t19792 - F::cast_from(0.57165357490759649296e-3_f64) * t7132 * t19682 + F::cast_from(0.47637797908966374413e-3_f64) * t7132 * t19688 - F::cast_from(0.47637797908966374413e-3_f64) * t25522 * t19693;
    t107012
}
