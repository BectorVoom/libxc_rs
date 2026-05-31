//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1561/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561<F: Float>(t1145: F, t141: F, t43797: F, t12327: F, t3391: F, t3399: F, t12322: F, t12343: F, t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F, t43783: F, t43785: F, t43787: F, t43791: F, t43795: F) -> (F, F, F, F) {
    let t43799 = t141 * t1145 * t43797;
    let t43802 = t12327 * t3391 * t3399;
    let t43804 = t12343 * t12322;
    let t43806 = -F::cast_from(0.98115555555555555555e-1_f64) * t43762 - F::cast_from(0.8585111111111111111e-1_f64) * t43769 - F::cast_from(0.98115555555555555556e0_f64) * t43771 + F::cast_from(0.44152e0_f64) * t43773 + F::cast_from(0.44152e0_f64) * t43779 + F::cast_from(0.5519e0_f64) * t43781 + F::cast_from(0.11038e1_f64) * t43783 - F::cast_from(0.22076e0_f64) * t43785 - F::cast_from(0.132456e1_f64) * t43787 - F::cast_from(0.99342e0_f64) * t43791 + F::cast_from(0.198684e1_f64) * t43795 + F::cast_from(0.82785e-1_f64) * t43799 + F::cast_from(0.11651625e2_f64) * t43802 - F::cast_from(0.51785e1_f64) * t43804;
    (t43799, t43802, t43804, t43806)
}
