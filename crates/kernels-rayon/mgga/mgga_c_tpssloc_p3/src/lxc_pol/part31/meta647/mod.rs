//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta647(t22893: f64, t23164: f64, t28345: f64, t23153: f64, t5544: f64, t6552: f64, t6637: f64, t16662: f64, t6638: f64, t28329: f64, t16927: f64, t87052: f64, t87529: f64, t23185: f64, t28426: f64, t81914: f64, t25248: f64, t776: f64, t87642: f64, t98336: f64, t28334: f64, t6547: f64, t28322: f64, t6579: f64, t16762: f64, t1888: f64, t6646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98345, t98349, t98353, t98356, t98359) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1920(t22893, t23164, t28345, t23153, t5544, t6552, t6637, t16662, t6638, t28329, t16927, t87052, t87529);
        let (t98363, t98367, t98374, t98380, t98384) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1921(t23185, t28426, t81914, t25248, t776, t87642, t98336, t28334, t6547, t28322, t6579, t16762, t1888, t6646);
    (t98345, t98349, t98353, t98356, t98359, t98363, t98367, t98374, t98380, t98384)
}
