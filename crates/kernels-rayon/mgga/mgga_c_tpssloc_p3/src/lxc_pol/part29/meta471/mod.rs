//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1806;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta471(t2379: f64, t28: f64, t2752: f64, t13487: f64, t1081: f64, t776: f64, t2553: f64, t2749: f64, t868: f64, t2745: f64, t1877: f64, t1915: f64, t22959: f64, t23286: f64, t23290: f64, t23295: f64, t2522: f64, t3231: f64, t4314: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23781, t23788) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1806(t2379, t28, t2752);
        let (t23789, t23792, t23796, t23807, t23810, t23813, t23820) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1807(t13487, t23788, t1081, t776, t2553, t28, t2749, t868, t2745, t1877, t1915, t22959, t23286, t23290, t23295, t23781, t2522, t3231, t4314, t6666, t6670, t6841, t6848);
    (t23781, t23788, t23789, t23792, t23796, t23807, t23810, t23813, t23820)
}
