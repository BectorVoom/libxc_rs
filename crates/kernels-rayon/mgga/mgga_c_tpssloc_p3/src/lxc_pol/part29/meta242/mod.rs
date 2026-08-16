//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1137;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1138;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta242(t1882: f64, t794: f64, t6562: f64, t225: f64, t258: f64, t852: f64, t214: f64, t1880: f64, t857: f64, t865: f64, t6553: f64, t1902: f64, t798: f64, t1887: f64, t206: f64, t6546: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6563, t6565, t6567, t6568, t6569, t6571) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1137(t1882, t794, t6562, t225, t258, t852, t214, t1880, t857);
        let t6572 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1138(t6571, t865);
        let (t6573, t6574, t6576, t6579) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1139(t6553, t6572, t1880, t1902, t798, t1887, t206, t6546);
    (t6563, t6565, t6567, t6568, t6569, t6571, t6572, t6573, t6574, t6576, t6579)
}
