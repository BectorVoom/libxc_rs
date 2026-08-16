//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 815/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk815(t16670: f64, t2992: f64, t1969: f64, t446: f64, t2983: f64, t9049: f64, t1882: f64, t4657: f64, t358: f64, t4714: f64, t363: f64, t4668: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16671 = t2992 * t16670;
    let t16672 = t1969 * t16671;
    let t16673 = t446 * t16672;
    let t16675 = t2983 * t16670;
    let t16676 = t9049 * t16675;
    let t16677 = t446 * t16676;
    let t16679 = t1882 * t4657;
    let t16681 = t4714 * t358;
    let t16682 = t16681 * t363;
    let t16683 = t1969 * t16682;
    let t16684 = t446 * t16683;
    let t16686 = t4668 * t358;
    (t16671, t16673, t16675, t16677, t16679, t16682, t16684, t16686)
}
