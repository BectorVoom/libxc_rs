//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 845/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk845(t6154: f64, t6921: f64, t729: f64, t6861: f64, t3977: f64, t7502: f64, t1091: f64, t724: f64, t7560: f64, t1901: f64, t33630: f64, t33636: f64, t35555: f64, t35559: f64, t35563: f64, t35567: f64, t35570: f64, t35574: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t35578 = t729 * t6154 * t6921;
    let t35582 = t729 * t6154 * t6861;
    let t35586 = t729 * t3977 * t7502;
    let t35590 = t724 * t7560 * t1091;
    let t35593 = 2.0_f64 / 3.0_f64 * t446 * t35555 - 2.0_f64 / 9.0_f64 * t1901 * t35559 + t1901 * t35563 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t35567 + 2.0_f64 / 9.0_f64 * t1901 * t35570 + 4.0_f64 / 3.0_f64 * t446 * t35574 + 2.0_f64 / 3.0_f64 * t446 * t35578 - t33630 + 2.0_f64 / 3.0_f64 * t446 * t35582 + 2.0_f64 / 3.0_f64 * t446 * t35586 + t33636 - t446 * t35590 / 9.0_f64;
    (t35578, t35582, t35586, t35590, t35593)
}
