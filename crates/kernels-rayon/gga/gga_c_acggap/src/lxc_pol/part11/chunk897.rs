//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 897/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk897(t30671: f64, t30225: f64, t425: f64, t1195: f64, t7605: f64, t1160: f64, t7560: f64, t3198: f64, t1170: f64, t30153: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30672 = 0.52832795046534975474e-1_f64 * t30671;
    let t30673 = t30225 * t425;
    let t30675 = t7605 * t1195;
    let t30689 = t1160 * t7560;
    let t30690 = t30689 * t3198;
    let t30692 = t1170 * t30153;
    (t30672, t30673, t30675, t30689, t30690, t30692)
}
