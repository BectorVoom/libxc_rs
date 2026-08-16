//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 635/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk635(t1268: f64, t2938: f64, t898: f64, t2946: f64, t3738: f64, t4967: f64, t4971: f64, t4975: f64, t5242: f64, t5245: f64, t900: f64, t2265: f64, t2912: f64, t4332: f64, t4350: f64, t5442: f64, t5446: f64, t5450: f64, t5454: f64, t631: f64) -> (f64, f64, f64, f64, f64) {
    let t5457 = t1268 * t1268;
    let t5459 = t898 * t2938 * t5457;
    let t5468 = -0.117377e0_f64 * t5242 + 0.234754e0_f64 * t5245 + t2946 + 0.9628722222222222222e-1_f64 * t3738 - 0.9628722222222222222e-1_f64 * t4967 + 0.28886166666666666666e0_f64 * t4971 - 0.14443083333333333333e0_f64 * t4975;
    let t5470 = t898 * t900 * t5468;
    let t5473 = -t2912 - 2.0_f64 / 9.0_f64 * t4332 - 2.0_f64 / 3.0_f64 * t4350 + t631 * t5442 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t2265 * t5446 - t631 * t5450 / 3.0_f64 + t631 * t5454 / 6.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t5459 + t631 * t5470 / 2.0_f64;
    (t5457, t5459, t5468, t5470, t5473)
}
