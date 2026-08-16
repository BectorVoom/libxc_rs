//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 941/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk941(t1248: f64, t2844: f64, t10697: f64, t2801: f64, t2843: f64, t4299: f64, t875: f64, t13301: f64, t835: f64, t3281: f64, t13320: f64, t2857: f64) -> (f64, f64, f64, f64, f64) {
    let t14602 = t1248 * t2844;
    let t14603 = t10697 * t14602;
    let t14607 = t1248 * t2801;
    let t14608 = t2843 * t14607;
    let t14615 = t4299 * t875;
    let t14616 = t2843 * t14615;
    let t14618 = t835 * t13301;
    let t14619 = t3281 * t14618;
    let t14621 = t2857 * t13320;
    (t14603, t14608, t14616, t14619, t14621)
}
