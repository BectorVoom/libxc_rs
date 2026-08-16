//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1945;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta660(t16918: f64, t23146: f64, t16898: f64, t4191: f64, t87199: f64, t4240: f64, t232: f64, t58569: f64, t6605: f64, t815: f64, t2628: f64, t5585: f64, t828: f64, t16949: f64, t221: f64, t25154: f64, t25119: f64, t841: f64, t25038: f64, t25248: f64, t776: f64, t98422: f64, t23110: f64, t23185: f64, t28321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98847, t98849, t98851, t98853, t98858, t98862) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1945(t16918, t23146, t16898, t4191, t87199, t4240, t232, t58569, t6605, t815, t2628, t5585, t828);
        let (t98868, t98871, t98881, t98884) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1946(t16949, t221, t25154, t25119, t841, t25038, t25248, t776, t98422, t23110, t23185, t28321);
    (t98847, t98849, t98851, t98853, t98858, t98862, t98868, t98871, t98881, t98884)
}
