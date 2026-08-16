//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1692/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1692(t1216: f64, t5971: f64, t11668: f64, t1090: f64, t6225: f64, t3578: f64, t11697: f64, t6191: f64, t3577: f64, t248: f64, t3570: f64, t6219: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18363 = t5971 * t1216;
    let t18364 = t11668 * t18363;
    let t18367 = t6225 * t1090;
    let t18368 = t3578 * t18367;
    let t18371 = t11697 * t6191;
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    (t18363, t18364, t18367, t18368, t18371, t18372, t18375)
}
