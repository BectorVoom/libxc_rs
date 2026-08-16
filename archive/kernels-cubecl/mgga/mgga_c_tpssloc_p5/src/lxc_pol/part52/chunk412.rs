//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 412/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk412<F: Float>(t1734: F, t475: F, t1214: F, t248: F, t46: F, t480: F, t47: F, t479: F, t471: F, t1230: F, t1653: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t467: F, t488: F) -> (F, F, F, F, F, F, F) {
    let t1735 = t1734 * t475;
    let t1737 = t248 * t1214 * t1735;
    let t1740 = t480 * t46;
    let t1742 = F::cast_from(1.0_f64) / t47 / t1740;
    let t1743 = t479 * t1742;
    let t1744 = t471 * t1743;
    let t1748 = t248 * t1230 * t1653;
    let t1751 = -t1706 * t467 / F::cast_from(36.0_f64) + t1195 - t1174 * t1726 / F::cast_from(288.0_f64) + t1731 * t488 / F::cast_from(3072.0_f64) + t1213 * t1737 / F::cast_from(3072.0_f64) - t1744 * t488 / F::cast_from(576.0_f64) + t1224 - t1227 * t1748 / F::cast_from(4608.0_f64);
    (t1735, t1737, t1742, t1743, t1744, t1748, t1751)
}
