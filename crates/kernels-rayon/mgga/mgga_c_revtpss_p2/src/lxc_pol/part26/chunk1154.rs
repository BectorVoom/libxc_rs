//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1154/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1154(t10209: f64, t94982: f64, t2366: f64, t665: f64, t25826: f64, t10254: f64, t6998: f64, t1450: f64, t9628: f64, t10426: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64) {
    let t94983 = t94982 * t10209;
    let t94985 = t665 * t2366;
    let t94986 = t25826 * t94985;
    let t94988 = t6998 * t10254;
    let t95002 = t1450 * t9628;
    let t95019 = t10426 * t196 * t197;
    (t94983, t94986, t94988, t95002, t95019)
}
