//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1260/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1260(t225: f64, t27052: f64, t2085: f64, t5286: f64, t1824: f64, t7191: f64, t12020: f64, t7213: f64, t112: f64, t27240: f64, t111: f64, t7945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93341 = t27052 * t225;
    let t93501 = t2085 * t5286;
    let t93505 = t7191 * t1824;
    let t93818 = t12020 * t7213;
    let t94127 = t27240 * t112;
    let t94170 = t7945 * t111;
    (t93341, t93501, t93505, t93818, t94127, t94170)
}
