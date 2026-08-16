//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta655(t17004: f64, t6581: f64, t16662: f64, t1894: f64, t236: f64, t6591: f64, t5568: f64, t81956: f64, t28389: f64, t81963: f64, t25068: f64, t4257: f64, t16853: f64, t6621: f64, t16946: f64, t16951: f64, t23053: f64, t5619: f64, t23083: f64, t28356: f64, t25093: f64, t7496: f64, t87504: f64, t25115: f64, t87451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98703, t98707, t98709, t98711, t98715) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1936(t17004, t6581, t16662, t1894, t236, t6591, t5568, t81956, t28389, t81963, t25068, t4257);
        let (t98717, t98719, t98721, t98723, t98725, t98728, t98731) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1937(t16853, t6621, t16946, t16951, t23053, t5619, t23083, t28356, t25093, t7496, t87504, t25115, t87451);
    (t98703, t98707, t98709, t98711, t98715, t98717, t98719, t98721, t98723, t98725, t98728, t98731)
}
