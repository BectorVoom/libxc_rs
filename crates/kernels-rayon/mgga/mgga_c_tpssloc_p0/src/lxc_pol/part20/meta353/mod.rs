//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1664;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1665;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta353(t3850: f64, t562: f64, t1352: f64, t12240: f64, t3806: f64, t5248: f64, t1339: f64, t836: f64, t1336: f64, t3809: f64, t3777: f64, t3789: f64, t12248: f64, t236: f64, t240: f64, t12251: f64, t1343: f64, t820: f64, t12255: f64, t3798: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12273, t12279, t12282, t12283) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1664(t3850, t562, t1352, t12240, t3806, t5248, t1339, t836, t1336);
        let (t12284, t12286, t12289, t12290, t12291, t12293, t12297) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1665(t12283, t3809, t3777, t3789, t12248, t236, t240, t1336, t12251, t1343, t820, t12255);
        let t12300 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1666(t3777, t3798);
    (t12273, t12279, t12282, t12283, t12284, t12286, t12289, t12290, t12291, t12293, t12297, t12300)
}
