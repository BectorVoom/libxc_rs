//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1517/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1517<F: Float>(t10678: F, t10682: F, t10687: F, t10692: F, t14759: F, t14761: F, t14765: F, t14769: F, t14774: F, t14777: F, t14780: F, t14783: F, t851: F) -> F {
    let t14784 = t14759 - F::cast_from(0.45178982497454656791e-5_f64) * t14761 - F::cast_from(0.60976381323476959249e-3_f64) * t10678 + F::cast_from(0.28582678745379824648e-4_f64) * t10682 - t10687 + t10692 - F::new(35.0) / F::new(216.0) * t14765 + F::cast_from(0.42874018118069736972e-2_f64) * t851 * t14769 - F::cast_from(0.25724410870841842183e-1_f64) * t851 * t14774 - F::cast_from(0.80031500487063509015e-2_f64) * t14777 + F::cast_from(0.10164000561857065645e-4_f64) * t14780 + t14783;
    t14784
}
