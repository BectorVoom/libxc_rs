//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1955;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta536(t5: f64, t26054: f64, t26095: f64, t112: f64, t1868: f64, t671: f64, t12725: f64, t1873: f64, t19456: f64, t4028: f64, t6534: f64, t1458: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26097, t26098, t26103) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1955(t5, t26054, t26095, t112, t1868, t671);
        let (t26109, t26111, t26113, t26114) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1956(t12725, t1873, t19456, t4028, t6534, t1458, t649);
    (t26097, t26098, t26103, t26109, t26111, t26113, t26114)
}
