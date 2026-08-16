//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1603;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1604;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1605;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta326(t11697: f64, t3579: f64, t3577: f64, t248: f64, t3494: f64, t3570: f64, t1213: f64, t3490: f64, t3523: f64, t1190: f64, t3030: f64, t3032: f64, t3505: f64, t10469: f64, t466: f64, t10471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11698, t11699, t11702, t11703, t11705, t11707, t11708) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1603(t11697, t3579, t3577, t248, t3494, t3570, t1213, t3490, t3523, t1190, t3030, t3032);
        let t11709 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1604(t11708, t3505);
        let t11712 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1605(t10469, t466);
        let t11713 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1606(t10471, t11712);
    (t11698, t11699, t11702, t11703, t11705, t11707, t11708, t11709, t11712, t11713)
}
