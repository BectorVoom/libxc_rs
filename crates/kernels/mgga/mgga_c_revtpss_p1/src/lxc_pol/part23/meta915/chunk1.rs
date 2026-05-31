//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2950/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950<F: Float>(t11385: F, t23467: F, t934: F, t11299: F, t4631: F, t6145: F, t23550: F, t41588: F, t23547: F, t2874: F, t23546: F, t2926: F) -> (F, F, F, F, F) {
    let t78319 = F::cast_from(0.57895126195293126241e3_f64) * t11385 * t23467 * t934;
    let t78322 = F::cast_from(0.28947563097646563121e3_f64) * t11299 * t6145 * t4631;
    let t78325 = F::cast_from(0.62071215503128080361e4_f64) * t41588 * t23550 * t934;
    let t78328 = F::cast_from(2.0_f64) * t2874 * t23547 * t934;
    let t78329 = t23546 * t2926;
    (t78319, t78322, t78325, t78328, t78329)
}
