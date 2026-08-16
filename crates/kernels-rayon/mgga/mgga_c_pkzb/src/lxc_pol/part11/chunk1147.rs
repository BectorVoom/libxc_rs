//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1147/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1147(t10282: f64, t914: f64, t10334: f64, t6455: f64, t10066: f64, t3206: f64, t6475: f64, t10195: f64, t178: f64, t915: f64, t10050: f64, t2380: f64) -> (f64, f64, f64, f64, f64) {
    let t26927 = t914 * t10282;
    let t26948 = t6455 * t10334;
    let t26970 = t3206 * t6475 * t10066;
    let t26975 = t915 * t10195 * t178;
    let t26981 = t2380 * t6475 * t10050;
    (t26927, t26948, t26970, t26975, t26981)
}
