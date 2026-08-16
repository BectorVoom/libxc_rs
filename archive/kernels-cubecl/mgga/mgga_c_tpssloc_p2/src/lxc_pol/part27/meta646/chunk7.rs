//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2226/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2226<F: Float>(t25806: F, t6680: F, t1955: F, t43603: F, t10160: F, t13735: F, t1409: F, t14548: F, t23327: F, t23329: F, t23330: F, t23336: F, t23346: F, t254: F, t25420: F, t25739: F, t25757: F, t25758: F, t25759: F, t25778: F, t25801: F, t3010: F, t3169: F, t3176: F, t3206: F, t4542: F, t6687: F, t6699: F, t7561: F, t7625: F, t83316: F, t83318: F, t991: F) -> F {
    let t88845 = F::cast_from(0.14621636149762012769e-1_f64) * t6680 * t25806;
    let t88851 = t43603 * t1955;
    let t88867 = -F::cast_from(0.87729816898572076613e-1_f64) * t23346 * t25739 - F::cast_from(0.54831135561607547884e-2_f64) * t83316 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23336 * t25801 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23329 * t23330 * t1409 * t3206 - F::cast_from(2.0_f64) * t10160 * t7625 - t88845 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3010 * t7561 + F::cast_from(4.0_f64) * t3169 * t25420 + F::cast_from(24.0_f64) * t25757 * t88851 * t13735 + F::cast_from(0.18277045187202515961e-2_f64) * t83318 - F::cast_from(12.0_f64) * t991 * t254 * t25759 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t6699 - F::cast_from(6.0_f64) * t25757 * t25758 * t14548 + F::cast_from(2.0_f64) * t25778 * t3176;
    t88867
}
