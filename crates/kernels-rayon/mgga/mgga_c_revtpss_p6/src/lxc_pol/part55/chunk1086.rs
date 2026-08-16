//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1086/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1086(t2014: f64, t34018: f64, t32629: f64, t7900: f64, t2089: f64, t7741: f64, t651: f64, t1518: f64, t8686: f64, t1502: f64, t7901: f64, t8698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34019 = t2014 * t34018;
    let t34021 = t32629 * t7900;
    let t34023 = 3.0_f64 * t2014 * t34021;
    let t34025 = t2089 * t7741;
    let t34027 = 2.0_f64 * t651 * t34025;
    let t34028 = t8686 * t1518;
    let t34030 = 2.0_f64 * t651 * t34028;
    let t34031 = t1502 * t8686;
    let t34033 = 3.0_f64 * t8698 * t7901;
    (t34019, t34021, t34023, t34025, t34027, t34028, t34030, t34031, t34033)
}
