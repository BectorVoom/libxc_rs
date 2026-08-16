//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1275/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1275<F: Float>(t131: F, t467: F, t50: F, t82510: F, t10469: F, t461: F, t11715: F, t11721: F, t3032: F, t3502: F, t3508: F, t11553: F, t2121: F, t2148: F) -> (F, F, F, F, F, F, F) {
    let t85963 = t50 * t82510 * t131 * t467;
    let t85964 = t461 * t10469;
    let t85965 = t85964 * t11715;
    let t85966 = t3032 * t11721;
    let t85971 = t85964 * t3502;
    let t85972 = t3032 * t3508;
    let t86000 = F::cast_from(0.30461741978670859935e-2_f64) * t2121 * t11553 * t2148;
    (t85963, t85964, t85965, t85966, t85971, t85972, t86000)
}
