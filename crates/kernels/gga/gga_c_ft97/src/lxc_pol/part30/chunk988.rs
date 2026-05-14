//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 988/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk988<F: Float>(t150709: F, t35952: F, t1701: F, t28633: F, t2035: F, t35924: F, t811: F, t820: F, t153047: F, t4092: F, t1200: F, t153116: F, t142810: F, t142815: F, t142818: F, t142820: F, t142823: F, t142832: F, t142833: F, t142834: F, t14721: F, t14742: F, t153094: F, t153113: F, t153118: F, t153121: F, t19101: F, t19132: F, t19135: F, t4065: F, t4110: F, t4115: F, t5264: F) -> (F, F, F) {
    let t153230 = t35952 * t150709;
    let t153241 = t1701 * t28633;
    let t153248 = t2035 * t35924 * t811;
    let t153256 = t2035 * t35924 * t820;
    let t153259 = t4092 * t153047;
    let t153262 = t1200 * t153116;
    let t153267 = -0.19592980390298668092e-1 * t153230 - t142810 + 0.20527106943485609994e0 * t5264 * t142832 * t142834 * t4115 - 0.20527106943485609994e0 * t142833 * t142834 * t4110 - 0.45306850413028723348e0 * t14721 * t153094 + 0.45306850413028723348e0 * t14742 * t153241 - 0.20527106943485609994e0 * t142833 * t142834 * t4065 + 0.10947790369858991997e2 * t19101 * t153248 - 0.80027204934668021496e-1 * t142815 + t142818 + 0.53351469956445347664e-1 * t142820 - t142823 - 0.54738951849294959986e1 * t19132 * t153248 + 0.27369475924647479993e1 * t19135 * t153256 - 0.84754336316176678532e-1 * t153259 * t153113 + 0.84754336316176678532e-1 * t153262 * t153118 - 0.45306850413028723348e0 * t14721 * t153121;
    (t153241, t153256, t153267)
}
