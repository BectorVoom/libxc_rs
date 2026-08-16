//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3089/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3089(t12621: f64, t1774: f64, t1214: f64, t16750: f64, t12629: f64, t3555: f64, t5412: f64, t1269: f64, t5216: f64, t3565: f64, t5215: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56543 = t1774 * t12621;
    let t56555 = t16750 * t1214;
    let t56561 = t1774 * t12629;
    let t56570 = t3555 * t5412;
    let t56575 = t5216 * t1269;
    let t56587 = t5215 * t3565;
    let t56588 = t56587 * t487;
    (t56543, t56555, t56561, t56570, t56575, t56587, t56588)
}
