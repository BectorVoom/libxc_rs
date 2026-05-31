//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 452/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk452<F: Float>(t1154: F, t1155: F, t1646: F, t1745: F, t304: F, t1152: F, t1153: F, t1757: F, t1761: F, t1780: F, t348: F, t365: F, t368: F, t86: F) -> (F, F, F) {
    let t1788 = t1154 * t1155 * t1646;
    let t1791 = t304 * t1745;
    let t1795 = F::cast_from(0.619125e-2_f64) * t1780 * t348 + F::cast_from(0.9286875e-2_f64) * t365 * t1757 - F::cast_from(0.619125e-2_f64) * t365 * t1761 - t1152 - F::cast_from(0.26531111111111111111e-1_f64) * t1153 * t1788 - F::cast_from(0.39796666666666666666e-1_f64) * t86 * t368 * t1791;
    (t1788, t1791, t1795)
}
