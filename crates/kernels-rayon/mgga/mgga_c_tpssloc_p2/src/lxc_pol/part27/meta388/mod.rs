//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1591;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1592;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1593;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1594;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta388(t14722: f64, t14704: f64, t11147: f64, t1409: f64, t2244: f64, t11145: f64, t123: f64, t11153: f64, t3240: f64, t3242: f64, t3966: f64, t607: f64, t2250: f64, t4723: f64, t1088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14723, t14724, t14726, t14728) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1591(t14722, t14704, t11147, t1409, t2244, t11145, t123);
        let (t14731, t14733) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1592(t11153, t1409, t2244, t3240, t123);
        let (t14736, t14738) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1593(t3242, t3966, t607, t3240, t123);
        let (t14740, t14742) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1594(t2250, t4723, t3240, t123);
        let (t14744, t14746) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1595(t2244, t4723, t1088, t123);
    (t14723, t14724, t14726, t14728, t14731, t14733, t14736, t14738, t14740, t14742, t14744, t14746)
}
