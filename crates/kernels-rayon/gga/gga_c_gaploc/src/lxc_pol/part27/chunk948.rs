//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 948/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk948(t10447: f64, t1445: f64, t1562: f64, t3354: f64, t4673: f64, t1572: f64, t3384: f64, t4950: f64, t10140: f64, t1457: f64, t3395: f64, t6985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10448 = t1445 * t10447;
    let t10450 = 0.69017266717057349418e1_f64 * t1562 * t10448;
    let t10455 = t4673 * t3354;
    let t10457 = 0.47667319935800568892e0_f64 * t1572 * t10455;
    let t10459 = 0.71500979903700853338e0_f64 * t4950 * t3384;
    let t10463 = t1457 * t10140;
    let t10465 = 0.71500979903700853338e0_f64 * t1572 * t10463;
    let t10466 = t6985 * t3395;
    (t10448, t10450, t10455, t10457, t10459, t10463, t10465, t10466)
}
