//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1173/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1173(t23763: f64, t31735: f64, t25722: f64, t6508: f64, t4261: f64, t9074: f64, t19532: f64, t25723: f64, t10163: f64, t1358: f64, t1367: f64, t31543: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31737 = 0.18970004423784099733e-1_f64 * t23763 * t31735;
    let t31752 = t6508 * t25722;
    let t31754 = t9074 * t4261 * t31752;
    let t31755 = 0.142275033178380748e-1_f64 * t31754;
    let t31757 = t9074 * t19532 * t25723;
    let t31758 = 0.71137516589190373998e-2_f64 * t31757;
    let t31759 = t1358 * t10163;
    let t31760 = 0.31616674039640166222e-2_f64 * t31759;
    let t31764 = t31543 * t1367;
    (t31737, t31752, t31755, t31758, t31760, t31764)
}
