//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2133;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta689(t26168: f64, t7685: f64, t19924: f64, t24995: f64, t8945: f64, t19456: f64, t7468: f64, t26003: f64, t4028: f64, t2314: f64, t28864: f64, t4034: f64, t1873: f64, t19289: f64, t652: f64, t1983: f64, t20085: f64, t6996: f64, t28827: f64, t6876: f64, t7684: f64, t8944: f64, t26164: f64, t75203: f64, t8643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96760, t96763, t96765, t96767, t96784, t96786) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2133(t26168, t7685, t19924, t24995, t8945, t19456, t7468, t26003, t4028, t2314, t28864, t4034);
        let (t96789, t96792, t96796, t96799, t96802) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2134(t1873, t19289, t652, t1983, t20085, t6996, t28827, t6876, t7684, t8944, t26164, t24995, t75203, t8643);
    (t96760, t96763, t96765, t96767, t96784, t96786, t96789, t96792, t96796, t96799, t96802)
}
