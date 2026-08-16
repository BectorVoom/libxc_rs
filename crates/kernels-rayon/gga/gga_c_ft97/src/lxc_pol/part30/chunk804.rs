//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 804/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk804(t1476: f64, t6386: f64, t840: f64, t871: f64, t1882: f64, t7631: f64, t1901: f64, t34062: f64, t34067: f64, t34070: f64, t34074: f64, t34078: f64, t34083: f64, t34086: f64, t34091: f64, t34095: f64, t34099: f64, t446: f64) -> (f64, f64, f64, f64) {
    let t34102 = t1476 * t6386;
    let t34104 = t840 * t871 * t34102;
    let t34108 = 2.0_f64 / 9.0_f64 * t1882 * t7631;
    let t34109 = 2.0_f64 / 3.0_f64 * t446 * t34062 + t446 * t34067 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t34070 - 4.0_f64 / 3.0_f64 * t1901 * t34074 - 4.0_f64 / 3.0_f64 * t1901 * t34078 - 2.0_f64 / 9.0_f64 * t1901 * t34083 + 2.0_f64 / 9.0_f64 * t1901 * t34086 + 4.0_f64 / 3.0_f64 * t446 * t34091 + 4.0_f64 / 3.0_f64 * t446 * t34095 - t446 * t34099 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t34104 - t34108;
    (t34102, t34104, t34108, t34109)
}
