//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2226/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2226(t25806: f64, t6680: f64, t1955: f64, t43603: f64, t10160: f64, t13735: f64, t1409: f64, t14548: f64, t23327: f64, t23329: f64, t23330: f64, t23336: f64, t23346: f64, t254: f64, t25420: f64, t25739: f64, t25757: f64, t25758: f64, t25759: f64, t25778: f64, t25801: f64, t3010: f64, t3169: f64, t3176: f64, t3206: f64, t4542: f64, t6687: f64, t6699: f64, t7561: f64, t7625: f64, t83316: f64, t83318: f64, t991: f64) -> f64 {
    let t88845 = 0.14621636149762012769e-1_f64 * t6680 * t25806;
    let t88851 = t43603 * t1955;
    let t88867 = -0.87729816898572076613e-1_f64 * t23346 * t25739 - 0.54831135561607547884e-2_f64 * t83316 - 0.54831135561607547884e-2_f64 * t23327 * t23336 * t25801 - 0.27415567780803773942e-2_f64 * t23327 * t23329 * t23330 * t1409 * t3206 - 2.0_f64 * t10160 * t7625 - t88845 - 0.82246703342411321825e-2_f64 * t6687 * t3010 * t7561 + 4.0_f64 * t3169 * t25420 + 24.0_f64 * t25757 * t88851 * t13735 + 0.18277045187202515961e-2_f64 * t83318 - 12.0_f64 * t991 * t254 * t25759 - 0.16449340668482264365e-1_f64 * t6687 * t4542 * t6699 - 6.0_f64 * t25757 * t25758 * t14548 + 2.0_f64 * t25778 * t3176;
    t88867
}
