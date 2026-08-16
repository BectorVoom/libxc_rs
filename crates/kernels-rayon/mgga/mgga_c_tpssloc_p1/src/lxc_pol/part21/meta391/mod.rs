//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta391 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta391(t12606: f64, t998: f64, t974: f64, t10868: f64, t1539: f64, t248: f64, t1041: f64, t1009: f64, t4552: f64, t1011: f64, t1019: f64, t1615: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14197, t14198, t14202, t14203, t14205, t14206, t14207) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1860(t12606, t998, t974, t10868, t1539, t248, t1041, t1009, t4552, t1011, t1019);
        let t14211 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1861(t1615, t3131);
    (t14197, t14198, t14202, t14203, t14205, t14206, t14207, t14211)
}
