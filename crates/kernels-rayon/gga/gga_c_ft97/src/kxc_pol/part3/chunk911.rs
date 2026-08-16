//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 911/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk911(t13643: f64, t18051: f64, t18055: f64, t18058: f64, t18061: f64, t18064: f64, t18066: f64, t18068: f64, t18071: f64, t18074: f64, t18077: f64, t18081: f64, t9637: f64) -> f64 {
    let t18083 = 0.6384360837962962963e-2_f64 * t18051 + 0.2269994964609053498e-1_f64 * t13643 - 0.51074886703703703704e-1_f64 * t18055 + 0.19862455940329218107e-1_f64 * t18058 - 0.34049924469135802469e-1_f64 * t18061 + 0.38306165027777777778e-1_f64 * t18064 + 0.6809984893827160494e-1_f64 * t18066 - 0.4539989929218106996e-1_f64 * t18068 + 0.51074886703703703704e-1_f64 * t18071 - 0.12768721675925925926e-1_f64 * t18074 + 0.85124811172839506173e-2_f64 * t18077 + t9637 + 0.62424861526748971193e-1_f64 * t18081;
    t18083
}
