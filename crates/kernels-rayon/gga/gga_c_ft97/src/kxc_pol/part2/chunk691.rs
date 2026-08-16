//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 691/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk691(t10975: f64, t446: f64, t3103: f64, t358: f64, t363: f64, t1564: f64, t1580: f64, t3008: f64, t1557: f64, t942: f64, t1559: f64, t7793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10976 = t446 * t10975;
    let t10978 = t3103 * t358;
    let t10979 = t10978 * t363;
    let t10980 = t1564 * t10979;
    let t10981 = t446 * t10980;
    let t10983 = t3008 * t1580;
    let t10984 = t1564 * t10983;
    let t10985 = t446 * t10984;
    let t10987 = t942 * t1557;
    let t10988 = t10987 * t1559;
    let t10989 = t7793 * t10988;
    (t10976, t10979, t10981, t10983, t10985, t10988, t10989)
}
