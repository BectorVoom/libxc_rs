//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1399/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1399(t10190: f64, t2990: f64, t2986: f64, t2770: f64, t607: f64, t2250: f64) -> (f64, f64, f64) {
    let t10191 = t10190 * t2990;
    let t10192 = t2986 * t10191;
    let t10194 = t2770 * t607;
    let t10195 = t10194 * t2250;
    (t10191, t10192, t10195)
}
