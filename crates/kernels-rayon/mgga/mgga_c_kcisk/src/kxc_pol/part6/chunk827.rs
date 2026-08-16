//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 827/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk827(t3521: f64, t7846: f64, t425: f64, t7757: f64, t7870: f64, t7862: f64, t2059: f64, t2083: f64, t7850: f64, t7854: f64, t1417: f64, t7879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26579 = t3521 * t7846;
    let t26590 = t425 * t7757;
    let t26600 = t3521 * t7870;
    let t26602 = t3521 * t7862;
    let t26617 = t2059 * t2083;
    let t26632 = t3521 * t7850;
    let t26692 = t3521 * t7854;
    let t26710 = t1417 * t7879;
    (t26579, t26590, t26600, t26602, t26617, t26632, t26692, t26710)
}
