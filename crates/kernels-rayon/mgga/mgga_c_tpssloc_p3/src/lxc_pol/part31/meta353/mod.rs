//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1266;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta353(t248: f64, t3521: f64, t4733: f64, t1227: f64, t3536: f64, t4997: f64, t3570: f64, t5012: f64, t1213: f64, t3535: f64, t5018: f64, t1202: f64, t5023: f64, t1742: f64, t3036: f64, t3503: f64, t3500: f64, t1210: f64, t11539: f64, t4724: f64, t1174: f64, t13969: f64, t4983: f64, t3515: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15488, t15490, t15494, t15495, t15498) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1266(t248, t3521, t4733, t1227, t3536, t4997, t3570, t5012, t1213, t3535, t5018, t1202, t5023);
        let (t15503, t15507, t15524, t15550, t15567) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1267(t1742, t3036, t3503, t3500, t1210, t11539, t4724, t1174, t13969, t4983, t3515, t478);
    (t15488, t15490, t15494, t15495, t15498, t15503, t15507, t15524, t15550, t15567)
}
