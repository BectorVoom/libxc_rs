//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 848/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk848(t1901: f64, t33658: f64, t33680: f64, t33682: f64, t35596: f64, t35601: f64, t35606: f64, t35610: f64, t35614: f64, t35617: f64, t35621: f64, t35625: f64, t35629: f64, t446: f64) -> f64 {
    let t35632 = t33658 + t446 * t35596 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t35601 - 2.0_f64 * t446 * t35606 - 4.0_f64 / 3.0_f64 * t1901 * t35610 - 4.0_f64 / 3.0_f64 * t1901 * t35614 + 2.0_f64 / 9.0_f64 * t1901 * t35617 + t1901 * t35621 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t35625 - t33680 + t33682 - t446 * t35629 / 3.0_f64;
    t35632
}
