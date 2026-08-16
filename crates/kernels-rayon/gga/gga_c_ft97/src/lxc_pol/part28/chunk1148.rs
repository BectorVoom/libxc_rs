//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1148/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1148(t139509: f64, t1969: f64, t446: f64, t920: f64, t18: f64, t3281: f64, t32962: f64, t9073: f64, t23657: f64, t27152: f64, t32924: f64, t9432: f64) -> (f64, f64, f64) {
    let t148563 = t446 * t1969 * t139509 * t920;
    let t148567 = t3281 * t9073 * t32962 * t18;
    let t148571 = t23657 * t9432 * t32924 * t27152;
    (t148563, t148567, t148571)
}
