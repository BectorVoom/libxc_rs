//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1039/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1039(t30468: f64, t4425: f64, t4685: f64, t7822: f64, t4331: f64, t1470: f64, t30644: f64, t30984: f64, t8458: f64, t2268: f64, t30456: f64, t1562: f64, t30948: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34500 = t30468 * t4425;
    let t34502 = t7822 * t4685;
    let t34504 = t7822 * t4331;
    let t34506 = t30644 * t1470;
    let t34508 = t30984 * t8458;
    let t34510 = t30456 * t2268;
    let t34512 = t30948 * t1562;
    (t34500, t34502, t34504, t34506, t34508, t34510, t34512)
}
