//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1108/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1108(t1843: f64, t1911: f64, t33578: f64, t33580: f64, t33583: f64, t34017: f64, t34019: f64, t34023: f64, t34027: f64, t34030: f64, t34031: f64, t34776: f64, t34788: f64, t508: f64, t569: f64, t8886: f64, t8897: f64) -> f64 {
    let t34790 = -t1843 * t8886 + t1911 * t8897 - t34776 * t508 + t34788 * t569 - t33578 - t33580 - t33583 - t34017 - t34019 + t34023 - t34027 - t34030 - t34031;
    t34790
}
