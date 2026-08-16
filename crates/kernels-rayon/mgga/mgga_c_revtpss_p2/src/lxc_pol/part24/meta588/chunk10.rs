//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1846/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1846(t198: f64, t3828: f64, t40076: f64, t40079: f64, t47122: f64, t47124: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t91875: f64, t92024: f64, t92026: f64, t92027: f64, t92028: f64, t92029: f64) -> f64 {
    let t92504 = 18.0_f64 * t198 * t3828 * t91875 + t40076 - t40079 + t47122 + t47124 + t47131 - t47138 - t47140 + t47142 + t47152 - t92024 + t92026 + t92027 + t92028 + t92029;
    t92504
}
