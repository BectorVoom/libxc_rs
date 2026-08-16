//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3198/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3198(t1774: f64, t487: f64, t1209: f64, t17807: f64, t3727: f64, t5219: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60037 = t487 * t1774;
    let t60087 = t1209 * t17807;
    let t60106 = t5219 * t3727;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    (t60037, t60087, t60106, t60221, t60224, t60248)
}
