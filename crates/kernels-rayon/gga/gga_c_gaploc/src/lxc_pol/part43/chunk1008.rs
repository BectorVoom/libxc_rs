//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1008/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1008(t1457: f64, t1572: f64, t46945: f64, t13728: f64, t4673: f64, t13805: f64, t1580: f64, t1445: f64, t47026: f64, t597: f64, t40455: f64, t40458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48124 = t1572 * t1457 * t46945;
    let t48127 = t1572 * t4673 * t13728;
    let t48131 = t1580 * t13805;
    let t48134 = t597 * t1445 * t47026;
    let t48137 = t597 * t1445 * t46945;
    let t48142 = 0.51123901271894332903e0_f64 * t40455;
    let t48143 = 0.38342925953920749677e0_f64 * t40458;
    (t48124, t48127, t48131, t48134, t48137, t48142, t48143)
}
