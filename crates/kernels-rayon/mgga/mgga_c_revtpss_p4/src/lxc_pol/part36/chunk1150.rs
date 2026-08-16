//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1150/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1150(t26054: f64, t5722: f64, t1955: f64, t7283: f64, t72: f64, t7920: f64, t686: f64, t25895: f64, t25878: f64, t1426: f64, t27836: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27861 = t26054 * t5722;
    let t27868 = t1955 * t7283;
    let t27872 = t7920 * t72;
    let t27873 = t27872 * t686;
    let t27874 = t25895 * t27873;
    let t27876 = t25878 * t27873;
    let t27883 = t27836 * t1426;
    let t27884 = t7063 * t27883;
    (t27861, t27868, t27872, t27873, t27874, t27876, t27883, t27884)
}
