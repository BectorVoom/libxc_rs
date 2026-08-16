//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 757/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk757(t1944: f64, t945: f64, t2530: f64, t795: f64, t740: f64, t2042: f64, t937: f64, t1881: f64, t954: f64, t2101: f64, t935: f64, t1891: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7141 = t1944 * t945;
    let t7143 = t795 * t2530;
    let t7144 = t7143 * t740;
    let t7147 = t2042 * t937;
    let t7152 = t954 * t1881;
    let t7157 = t2101 * t935;
    let t7158 = t7157 * t1891;
    (t7141, t7143, t7144, t7147, t7152, t7157, t7158)
}
