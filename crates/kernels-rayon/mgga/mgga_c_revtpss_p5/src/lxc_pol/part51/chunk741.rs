//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 741/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk741(t1982: f64, t8507: f64, t1984: f64, t359: f64, t1981: f64, t338: f64, t3056: f64) -> (f64, f64, f64, f64) {
    let t8508 = t1982 * t8507;
    let t8509 = t1984 * t359;
    let t8512 = t1981 * t338;
    let t8513 = t8512 * t3056;
    (t8508, t8509, t8512, t8513)
}
