//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1166/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1166<F: Float>(t1470: F, t30540: F, t1549: F, t30644: F, t1554: F, t1558: F, t4326: F, t7647: F, t1421: F, t1983: F, t30827: F, t7586: F) -> (F, F, F, F, F, F) {
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35976 = F::new(0.17149607247227894789e-2) * t35975;
    let t35977 = t30644 * t1554;
    let t35978 = F::new(0.17149607247227894789e-2) * t35977;
    let t35979 = t30644 * t1558;
    let t35980 = F::new(0.85748036236139473944e-3) * t35979;
    let t35981 = t7647 * t4326;
    let t35982 = F::new(0.85748036236139473944e-3) * t35981;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    (t35973, t35976, t35978, t35980, t35982, t35985)
}
