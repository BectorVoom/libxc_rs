//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 656/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk656(t12: f64, t171: f64, t8946: f64, t341: f64, t630: f64, t343: f64, t70: f64, t120: f64, t358: f64, t363: f64, t123: f64, t532: f64, t7911: f64) -> (f64, f64, f64, f64, f64) {
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    let t8965 = t120 * t358;
    let t8966 = t8965 * t363;
    let t8977 = t123 / t532 / t7911;
    (t8948, t8959, t8963, t8966, t8977)
}
