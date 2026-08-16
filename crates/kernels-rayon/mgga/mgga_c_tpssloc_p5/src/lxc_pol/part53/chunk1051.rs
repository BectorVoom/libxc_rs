//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1051/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1051(t117006: f64, t117084: f64, t122698: f64, t124476: f64, t124531: f64, t124540: f64, t1458: f64, t1849: f64, t19577: f64, t2040: f64, t2075: f64, t22574: f64, t2314: f64, t23938: f64, t25988: f64, t26161: f64, t26163: f64, t26875: f64, t26977: f64, t27170: f64, t27219: f64, t27226: f64, t32108: f64, t32278: f64, t33363: f64, t33883: f64, t4034: f64, t5361: f64, t574: f64, t652: f64, t7042: f64, t7156: f64, t7171: f64, t7787: f64, t7802: f64, t8780: f64, t9003: f64, t92090: f64) -> f64 {
    let t124552 = -4.0_f64 * t92090 * t2040 + 2.0_f64 * t26161 * t124476 * t26163 + 12.0_f64 * t122698 * t26875 - 3.0_f64 * t22574 * t117084 * t19577 + t32278 * t1849 + t8780 * t5361 - 2.0_f64 * t2314 * t33883 - 2.0_f64 * t4034 * t33883 - 2.0_f64 * t652 * t32108 * t1458 - 4.0_f64 * t652 * t2075 * t27170 - 4.0_f64 * t23938 * t7802 - 4.0_f64 * t26977 * t7802 - 4.0_f64 * t7042 * t27226 + (t124531 + t124540) * t574 - 2.0_f64 * t7787 * t7156 + 6.0_f64 * t33363 * t7171 - 4.0_f64 * t9003 * t27219 + 6.0_f64 * t22574 * t117006 * t25988;
    t124552
}
