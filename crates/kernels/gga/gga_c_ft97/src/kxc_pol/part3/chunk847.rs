//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 847/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk847<F: Float>(t10328: F, t19168: F, t1095: F, t274: F, t231: F, t3750: F, t230: F, t4939: F, t803: F, t5252: F, t688: F, t1193: F, t5255: F, t18127: F, t278: F, t10339: F, t10355: F, t18826: F, t18831: F, t18834: F, t19151: F, t19155: F, t19163: F, t19167: F, t2014: F, t2394: F, t2710: F, t4068: F, t4069: F, t807: F, t8948: F, t8963: F, t9609: F) -> (F,) {
    let t19169 = t19168 * t10328;
    let t19172 = t1095 * t274;
    let t19174 = t231 * t19172 * t3750;
    let t19177 = t230 * t4939;
    let t19178 = t19177 * t803;
    let t19181 = t5252 * t688;
    let t19184 = t1193 * t3750;
    let t19189 = t5255 * t688;
    let t19192 = t18127 * t278;
    let t19201 = 0.17557713923258613e0 * t18826 * t4069 - 0.23410285231011484e0 * t4068 * t19151 + 0.33205381699090447729e-3 * t8948 * t19155 - 0.11705142615505742e0 * t18831 * t4069 + 0.23410285231011484e0 * t18834 * t274 - 0.26564305359272358183e-2 * t2014 * t19163 - t19167 - 0.8854768453090786061e-3 * t8963 * t19169 - 0.53128610718544716366e-2 * t2014 * t19174 + 0.72343824494974941953e-3 * t8963 * t19178 - 0.5116527820486904976e-1 * t10339 * t19181 + 0.639565977560863122e-1 * t2710 * t19184 - 0.25159457085530922489e-1 * t9609 * t19181 + 0.319782988780431561e-1 * t2710 * t19189 - 0.532971647967385935e-1 * t807 * t19192 + 0.13977476158628290272e-1 * t2394 * t19189 + 0.27954952317256580544e-1 * t2394 * t19184 - 0.91641760171536135284e-3 * t10355 * t19181;
    (t19201,)
}
