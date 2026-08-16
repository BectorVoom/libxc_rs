//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1107/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1107(t117: f64, t34775: f64, t1518: f64, t2055: f64, t29427: f64, t33287: f64, t33644: f64, t33646: f64, t34308: f64, t34310: f64, t34312: f64, t34320: f64, t34323: f64, t34325: f64, t34446: f64, t7586: f64, t7983: f64, t8564: f64) -> (f64, f64) {
    let t34776 = t34775 * t117;
    let t34788 = 2.0_f64 * t1518 * t33287 + 2.0_f64 * t2055 * t29427 + 2.0_f64 * t2055 * t34446 + 2.0_f64 * t7586 * t7983 + t33644 + t33646 + t34308 + t34310 + t34312 + t34320 + t34323 + t34325 + t34776 + t8564;
    (t34776, t34788)
}
