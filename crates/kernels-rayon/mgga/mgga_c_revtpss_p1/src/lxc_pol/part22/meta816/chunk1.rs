//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2926/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2926(t2453: f64, t3908: f64, t5711: f64, t14296: f64, t9303: f64, t13738: f64, t686: f64, t72: f64, t9680: f64, t213: f64, t556: f64, t1903: f64, t9656: f64) -> (f64, f64, f64, f64, f64) {
    let t47784 = t2453 * t5711 * t3908;
    let t47786 = t9303 * t14296;
    let t47791 = t9680 * t13738 * t72 * t686;
    let t47793 = t213 * t556;
    let t47794 = t9656 * t1903;
    (t47784, t47786, t47791, t47793, t47794)
}
