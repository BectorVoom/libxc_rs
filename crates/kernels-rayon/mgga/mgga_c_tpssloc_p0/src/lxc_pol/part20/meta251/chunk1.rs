//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1375/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1375(t225: f64, t9725: f64, t9877: f64, t9908: f64, t9935: f64, t1891: f64, t68: f64, t9458: f64, t776: f64, t845: f64, t2553: f64, t824: f64, t9516: f64) -> (f64, f64, f64, f64) {
    let t9938 = (t9725 + t9877 + t9908 + t9935) * t225;
    let t9946 = t68 * t1891;
    let t9947 = t9946 * t9458;
    let t9950 = t845 * t776;
    let t9951 = t9950 * t2553;
    let t9954 = t824 * t9516;
    (t9938, t9947, t9951, t9954)
}
