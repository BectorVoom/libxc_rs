//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1154/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1154(t1984: f64, t25586: f64, t359: f64, t3057: f64, t7143: f64, t7145: f64, t7146: f64, t999: f64, t1096: f64, t7152: f64, t7160: f64, t1035: f64, t8515: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25588 = t1984 * t359 * t25586;
    let t25591 = t3057 * t7143;
    let t25593 = t7145 * t7146 * t999;
    let t25596 = t7152 * t1096;
    let t25597 = t7160 * t25596;
    let t25601 = t7160 * t7146 * t1096;
    let t25604 = t8515 * t1035;
    (t25588, t25591, t25593, t25596, t25597, t25601, t25604)
}
