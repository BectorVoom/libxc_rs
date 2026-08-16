//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2231/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2231(t23511: f64, t7577: f64, t23665: f64, t25524: f64, t23384: f64, t25518: f64, t13611: f64, t23346: f64, t23601: f64, t23670: f64, t23679: f64, t25476: f64, t6687: f64, t6784: f64, t6785: f64, t82562: f64, t82564: f64, t82574: f64, t82576: f64, t82590: f64, t82605: f64) -> f64 {
    let t89044 = t7577 * t23511;
    let t89049 = 0.54831135561607547884e-2_f64 * t23665 * t25524;
    let t89057 = 0.18277045187202515961e-2_f64 * t23384 * t25518;
    let t89066 = -0.16449340668482264365e-1_f64 * t23601 * t89044 * t23679 - t89049 + 0.43864908449286038306e-1_f64 * t23670 * t25524 + 0.91385225936012579807e-3_f64 * t82562 + 0.12184696791468343974e-2_f64 * t82564 - 0.48738787165873375897e-2_f64 * t82574 + 0.18277045187202515961e-2_f64 * t82576 + t89057 - 0.14621636149762012769e-1_f64 * t23346 * t25476 + 0.27415567780803773942e-2_f64 * t6687 * t6784 * t6785 * t13611 - 0.54831135561607547884e-2_f64 * t82590 - 0.27415567780803773942e-2_f64 * t82605;
    t89066
}
