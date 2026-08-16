//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk711;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta153(t3441: f64, t5392: f64, t3440: f64, t4904: f64, t4919: f64, t3455: f64, t1177: f64, t1178: f64, t5398: f64, t3464: f64, t4770: f64, t6012: f64, t6015: f64, t6018: f64, t457: f64, t460: f64, t974: f64, t1714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6119, t6120, t6123, t6126, t6127, t6130, t6131, t6138) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk711(t3441, t5392, t3440, t4904, t4919, t3455, t1177, t1178, t5398, t3464, t4770, t6012, t6015, t6018);
        let (t6140, t6141, t6144) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk712(t457, t6138, t460, t974, t1714);
    (t6119, t6120, t6123, t6126, t6127, t6130, t6131, t6138, t6140, t6141, t6144)
}
