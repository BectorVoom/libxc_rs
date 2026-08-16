//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1166/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1166(t118006: f64, t24744: f64, t24721: f64, t7330: f64, t7337: f64, t24711: f64, t8875: f64, t32514: f64, t7294: f64, t2144: f64, t7319: f64, t1170: f64, t2121: f64, t32503: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118007 = t24744 * t118006;
    let t118017 = t24721 * t7337 * t7330;
    let t118019 = t24711 * t8875;
    let t118034 = t7294 * t32514;
    let t118038 = t7319 * t2144;
    let t118050 = t2121 * t1170 * t32503;
    (t118007, t118017, t118019, t118034, t118038, t118050)
}
