//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 909/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk909(t1175: f64, t2409: f64, t724: f64, t2413: f64, t3897: f64, t2599: f64, t2405: f64, t9803: f64, t1934: f64, t992: f64, t3892: f64, t3891: f64) -> (f64, f64, f64, f64, f64) {
    let t14064 = t724 * t1175 * t2409;
    let t14067 = t3897 * t2413;
    let t14068 = t2599 * t14067;
    let t14071 = t3897 * t2405;
    let t14072 = t9803 * t14071;
    let t14075 = t992 * t1934;
    let t14076 = t3892 * t14075;
    let t14077 = t3891 * t14076;
    (t14064, t14068, t14072, t14075, t14077)
}
