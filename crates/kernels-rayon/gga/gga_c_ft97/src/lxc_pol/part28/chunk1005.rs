//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1005/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1005(t7165: f64, t984: f64, t1286: f64, t1310: f64, t136072: f64, t136075: f64, t144503: f64, t144551: f64, t1564: f64, t2: f64, t22873: f64, t22935: f64, t25535: f64, t25553: f64, t25564: f64, t25577: f64, t26: f64, t28: f64, t3052: f64, t32016: f64, t32423: f64, t34352: f64, t34553: f64, t34614: f64, t34620: f64, t379: f64, t4: f64, t5495: f64, t5501: f64, t5620: f64, t7162: f64, t7824: f64) -> f64 {
    let t144562 = t7165 * t984;
    let t144569 = -2.0_f64 / 9.0_f64 * t25577 * t1564 * t32423 * t3052 + t32016 * t25564 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t22873 * t34352 - t7162 * t25535 / 3.0_f64 + t136072 / 9.0_f64 + t34614 * t5620 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t136075 + (t144503 + t144551) * t2 * t4 * t26 * t1310 / 6.0_f64 + t7162 * t25553 / 6.0_f64 - t5495 * t34620 / 3.0_f64 + t5501 * t7824 * t144562 * t379 / 9.0_f64 - t22935 * t34553 / 18.0_f64;
    t144569
}
