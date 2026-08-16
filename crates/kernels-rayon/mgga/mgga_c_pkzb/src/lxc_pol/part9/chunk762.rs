//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 762/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk762(t51: f64, t5373: f64, t164: f64, t592: f64, t1727: f64, t1760: f64, t1717: f64, t1726: f64, t1723: f64, t5295: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5374 = t51 * t5373;
    let t5376 = t592 * t5374 * t164;
    let t5379 = t1727 * t1760;
    let t5381 = t1717 * t1726;
    let t5382 = t5381 * t1723;
    let t5384 = t588 * t5295;
    (t5374, t5376, t5379, t5381, t5382, t5384)
}
