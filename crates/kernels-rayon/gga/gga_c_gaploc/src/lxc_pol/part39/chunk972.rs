//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 972/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk972(t1063: f64, t2343: f64, t2787: f64, t30208: f64, t12834: f64, t6305: f64, t2268: f64, t9493: f64, t988: f64, t12763: f64, t6313: f64, t12770: f64, t2312: f64) -> (f64, f64, f64, f64, f64) {
    let t42737 = 0.56910013271352299198e-1_f64 * t1063 * t2343 * t2787 * t30208;
    let t42739 = 0.28455006635676149599e-1_f64 * t6305 * t12834;
    let t42742 = 0.28455006635676149599e-1_f64 * t2268 * t9493 * t988;
    let t42743 = t6313 * t12763;
    let t42745 = t2312 * t12770;
    (t42737, t42739, t42742, t42743, t42745)
}
