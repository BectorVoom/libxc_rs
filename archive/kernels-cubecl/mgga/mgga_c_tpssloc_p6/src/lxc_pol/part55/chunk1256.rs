//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1256/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1256<F: Float>(t12524: F, t33193: F, t20173: F, t33188: F, t3941: F, t6534: F, t7467: F, t1873: F, t26135: F, t4072: F, t576: F, t8319: F) -> (F, F, F, F, F) {
    let t120818 = F::cast_from(27.0_f64) * t12524 * t33193;
    let t120820 = F::cast_from(54.0_f64) * t20173 * t33188;
    let t120823 = F::cast_from(54.0_f64) * t3941 * t6534 * t7467;
    let t120830 = F::cast_from(54.0_f64) * t3941 * t1873 * t26135;
    let t120833 = t576 * t4072;
    let t120835 = F::cast_from(27.0_f64) * t120833 * t8319;
    (t120818, t120820, t120823, t120830, t120835)
}
