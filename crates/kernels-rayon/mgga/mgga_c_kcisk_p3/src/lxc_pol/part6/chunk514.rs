//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 514/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk514(t1417: f64, t2222: f64, t1173: f64, t416: f64, t2083: f64, t459: f64, t425: f64, t2226: f64, t458: f64, t2233: f64, t3739: f64, t2237: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5918 = t1417 * t2222;
    let t5926 = t416 * t1173;
    let t5927 = t459 * t2083;
    let t5932 = t425 * t2083;
    let t5941 = t1417 * t2226;
    let t5953 = t416 * t458;
    let t5972 = t3739 * t2233;
    let t5979 = t3739 * t2237;
    (t5918, t5926, t5927, t5932, t5941, t5953, t5972, t5979)
}
