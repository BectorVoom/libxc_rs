//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1128/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1128<F: Float>(t142810: F, t142815: F, t142818: F, t142820: F, t142823: F, t142832: F, t142833: F, t142834: F, t14721: F, t14742: F, t153094: F, t153113: F, t153118: F, t153121: F, t153230: F, t153241: F, t153248: F, t153256: F, t153259: F, t153262: F, t19101: F, t19132: F, t19135: F, t4065: F, t4110: F, t4115: F, t5264: F) -> F {
    let t153267 = -F::cast_from(0.19592980390298668092e-1_f64) * t153230 - t142810 + F::cast_from(0.20527106943485609994e0_f64) * t5264 * t142832 * t142834 * t4115 - F::cast_from(0.20527106943485609994e0_f64) * t142833 * t142834 * t4110 - F::cast_from(0.45306850413028723348e0_f64) * t14721 * t153094 + F::cast_from(0.45306850413028723348e0_f64) * t14742 * t153241 - F::cast_from(0.20527106943485609994e0_f64) * t142833 * t142834 * t4065 + F::cast_from(0.10947790369858991997e2_f64) * t19101 * t153248 - F::cast_from(0.80027204934668021496e-1_f64) * t142815 + t142818 + F::cast_from(0.53351469956445347664e-1_f64) * t142820 - t142823 - F::cast_from(0.54738951849294959986e1_f64) * t19132 * t153248 + F::cast_from(0.27369475924647479993e1_f64) * t19135 * t153256 - F::cast_from(0.84754336316176678532e-1_f64) * t153259 * t153113 + F::cast_from(0.84754336316176678532e-1_f64) * t153262 * t153118 - F::cast_from(0.45306850413028723348e0_f64) * t14721 * t153121;
    t153267
}
