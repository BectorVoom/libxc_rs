//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1495/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1495(t10481: f64, t3131: f64, t1021: f64, t248: f64, t1015: f64, t10478: f64, t10472: f64) -> (f64, f64, f64, f64) {
    let t10877 = t10481 * t3131;
    let t10879 = t248 * t1021 * t10877;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    (t10877, t10879, t10882, t10883)
}
