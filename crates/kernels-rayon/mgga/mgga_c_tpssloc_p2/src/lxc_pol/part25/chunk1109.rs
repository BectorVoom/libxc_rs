//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1109/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1109(t12156: f64, t6637: f64, t6968: f64, t80732: f64, t1372: f64, t1992: f64, t3850: f64, t550: f64, t6976: f64, t3791: f64, t22700: f64, t6914: f64) -> (f64, f64, f64, f64, f64) {
    let t81087 = t80732 * t6637 * t6968 * t12156;
    let t81092 = t1992 * t6976 * t1372 * t3850 * t550;
    let t81094 = t1372 * t3791;
    let t81097 = t1992 * t6976 * t81094 * t550;
    let t81099 = t6914 * t22700;
    (t81087, t81092, t81094, t81097, t81099)
}
