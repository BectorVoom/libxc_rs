//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1853;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta433(t4677: f64, t4684: f64, t14506: f64, t3185: f64, t1932: f64, t3120: f64, t360: f64, t1629: f64, t1625: f64, t3040: f64, t3201: f64, t6739: f64, t14526: f64, t383: f64, t1022: f64, t4657: f64, t1060: f64, t3188: f64, t1057: f64, t14205: f64, t11054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14615, t14618, t14622, t14623, t14626, t14627, t14630) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1853(t4677, t4684, t14506, t3185, t1932, t3120, t360, t1629, t1625, t3040, t3201, t6739);
        let (t14631, t14640, t14645, t14648, t14651, t14654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1854(t14630, t1629, t14526, t383, t1022, t4657, t1060, t14626, t3188, t1057, t14205, t11054);
    (t14615, t14618, t14622, t14623, t14627, t14630, t14631, t14640, t14645, t14648, t14651, t14654)
}
