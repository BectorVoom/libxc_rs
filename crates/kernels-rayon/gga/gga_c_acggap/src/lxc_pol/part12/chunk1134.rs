//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1134/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1134(t1423: f64, t7746: f64, t4640: f64, t7332: f64, t4645: f64, t570: f64, t1507: f64, t2020: f64, t30120: f64, t8793: f64, t1165: f64, t33735: f64, t604: f64, t7413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36139 = t7746 * t1423;
    let t36147 = t7332 * t4640;
    let t36149 = t570 * t4645;
    let t36151 = t2020 * t1507;
    let t36156 = t30120 * t8793;
    let t36160 = t7413 * t1165 * t604 * t33735;
    (t36139, t36147, t36149, t36151, t36156, t36160)
}
