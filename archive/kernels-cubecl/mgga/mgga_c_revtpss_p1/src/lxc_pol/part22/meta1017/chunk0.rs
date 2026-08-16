//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3516/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3516<F: Float>(t11773: F, t4954: F, t1011: F, t6284: F, t697: F, t19900: F, t3241: F, t11883: F, t12004: F, t16223: F, t19707: F, t19917: F, t42740: F, t42745: F, t42756: F, t54198: F, t54222: F, t54259: F, t54857: F, t6285: F, t6331: F) -> (F, F) {
    let t66542 = t4954 * t11773;
    let t66547 = t1011 * t697 * t6284;
    let t66551 = t3241 * t19900;
    let t66558 = -F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t42740 - t42745 - F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t42756 - F::cast_from(0.60976381323476959249e-2_f64) * t54198 - F::cast_from(0.96545937095505185476e-2_f64) * t12004 * t6331 + F::cast_from(0.95275595817932748826e-3_f64) * t66542 * t16223 - F::cast_from(0.20325460441158986416e-2_f64) * t54222 + t66547 / F::cast_from(648.0_f64) - F::cast_from(11.0_f64) / F::cast_from(162.0_f64) * t11883 * t6285 + t66551 / F::cast_from(81.0_f64) - t3241 * t19917 / F::cast_from(54.0_f64) + F::cast_from(0.11433071498151929859e-2_f64) * t54857 * t19707 - F::cast_from(0.60976381323476959249e-2_f64) * t54259;
    (t66542, t66558)
}
