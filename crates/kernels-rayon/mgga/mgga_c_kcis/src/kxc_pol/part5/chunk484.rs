//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 484/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk484(t1396: f64, t2001: f64, t1468: f64, t1464: f64, t1929: f64, t556: f64, t553: f64, t303: f64, t1650: f64, t8: f64, t168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2002 = t1396 * t2001;
    let t2003 = t1468 * t2002;
    let t2004 = t1464 * t2003;
    let t2006 = t1929 * t556;
    let t2007 = t553 * t2006;
    let t2008 = t303 * t2007;
    let t2010 = t8 * t1650;
    let t2011 = 1.0_f64 - t168 + t2010;
    (t2002, t2003, t2004, t2006, t2007, t2008, t2011)
}
