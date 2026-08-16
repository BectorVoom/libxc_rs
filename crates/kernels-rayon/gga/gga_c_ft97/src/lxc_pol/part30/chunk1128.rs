//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1128/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1128(t142810: f64, t142815: f64, t142818: f64, t142820: f64, t142823: f64, t142832: f64, t142833: f64, t142834: f64, t14721: f64, t14742: f64, t153094: f64, t153113: f64, t153118: f64, t153121: f64, t153230: f64, t153241: f64, t153248: f64, t153256: f64, t153259: f64, t153262: f64, t19101: f64, t19132: f64, t19135: f64, t4065: f64, t4110: f64, t4115: f64, t5264: f64) -> f64 {
    let t153267 = -0.19592980390298668092e-1_f64 * t153230 - t142810 + 0.20527106943485609994e0_f64 * t5264 * t142832 * t142834 * t4115 - 0.20527106943485609994e0_f64 * t142833 * t142834 * t4110 - 0.45306850413028723348e0_f64 * t14721 * t153094 + 0.45306850413028723348e0_f64 * t14742 * t153241 - 0.20527106943485609994e0_f64 * t142833 * t142834 * t4065 + 0.10947790369858991997e2_f64 * t19101 * t153248 - 0.80027204934668021496e-1_f64 * t142815 + t142818 + 0.53351469956445347664e-1_f64 * t142820 - t142823 - 0.54738951849294959986e1_f64 * t19132 * t153248 + 0.27369475924647479993e1_f64 * t19135 * t153256 - 0.84754336316176678532e-1_f64 * t153259 * t153113 + 0.84754336316176678532e-1_f64 * t153262 * t153118 - 0.45306850413028723348e0_f64 * t14721 * t153121;
    t153267
}
