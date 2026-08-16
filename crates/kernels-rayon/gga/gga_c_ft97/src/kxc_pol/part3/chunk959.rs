//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 959/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk959(t1091: f64, t2923: f64, t4370: f64, t2253: f64, t5470: f64, t5459: f64, t10304: f64, t4939: f64, t2697: f64, t4977: f64, t18127: f64, t801: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18820 = t2923 * t1091 * t4370;
    let t18823 = t2253 * t5470;
    let t18825 = t2253 * t5459;
    let t18826 = t10304 * t4939;
    let t18831 = t2697 * t4977;
    let t18834 = t801 * t18127;
    (t18820, t18823, t18825, t18826, t18831, t18834)
}
