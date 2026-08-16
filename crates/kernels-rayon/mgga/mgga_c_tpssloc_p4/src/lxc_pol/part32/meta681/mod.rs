//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta681 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta681(t1404: f64, t8110: f64, t1851: f64, t7426: f64, t27907: f64, t580: f64, t2169: f64, t5381: f64, t1395: f64, t8119: f64, t1858: f64, t7415: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t96283, t96285, t96289, t96291, t96300, t96303) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2121(t1404, t8110, t1851, t7426, t27907, t580, t2169, t5381, t1395, t8119, t1858, t7415);
    (t96283, t96285, t96289, t96291, t96300, t96303)
}
