//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 980/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk980<F: Float>(t10339: F, t10355: F, t18826: F, t18831: F, t18834: F, t19151: F, t19155: F, t19163: F, t19167: F, t19169: F, t19174: F, t19178: F, t19181: F, t19184: F, t19189: F, t19192: F, t2014: F, t2394: F, t2710: F, t274: F, t4068: F, t4069: F, t807: F, t8948: F, t8963: F, t9609: F) -> F {
    let t19201 = F::cast_from(0.17557713923258613e0_f64) * t18826 * t4069 - F::cast_from(0.23410285231011484e0_f64) * t4068 * t19151 + F::cast_from(0.33205381699090447729e-3_f64) * t8948 * t19155 - F::cast_from(0.11705142615505742e0_f64) * t18831 * t4069 + F::cast_from(0.23410285231011484e0_f64) * t18834 * t274 - F::cast_from(0.26564305359272358183e-2_f64) * t2014 * t19163 - t19167 - F::cast_from(0.8854768453090786061e-3_f64) * t8963 * t19169 - F::cast_from(0.53128610718544716366e-2_f64) * t2014 * t19174 + F::cast_from(0.72343824494974941953e-3_f64) * t8963 * t19178 - F::cast_from(0.5116527820486904976e-1_f64) * t10339 * t19181 + F::cast_from(0.639565977560863122e-1_f64) * t2710 * t19184 - F::cast_from(0.25159457085530922489e-1_f64) * t9609 * t19181 + F::cast_from(0.319782988780431561e-1_f64) * t2710 * t19189 - F::cast_from(0.532971647967385935e-1_f64) * t807 * t19192 + F::cast_from(0.13977476158628290272e-1_f64) * t2394 * t19189 + F::cast_from(0.27954952317256580544e-1_f64) * t2394 * t19184 - F::cast_from(0.91641760171536135284e-3_f64) * t10355 * t19181;
    t19201
}
