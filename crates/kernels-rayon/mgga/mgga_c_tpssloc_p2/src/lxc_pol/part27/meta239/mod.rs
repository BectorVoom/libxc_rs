//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1148;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1149;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1150;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta239(t6553: f64, t6555: f64, t6552: f64, t154: f64, t16: f64, t117: f64, t206: f64, t67: f64, t1882: f64, t794: f64, t225: f64, t258: f64, t852: f64, t214: f64, t1880: f64, t857: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6556, t6557, t6559) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1148(t6553, t6555, t6552, t154, t16);
        let (t6561, t6562) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1149(t117, t206, t67, t6559);
        let (t6563, t6565, t6567, t6568, t6569, t6571) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1150(t1882, t794, t6562, t225, t258, t852, t214, t1880, t857);
        let t6572 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1151(t6571, t865);
    (t6556, t6557, t6559, t6561, t6562, t6563, t6565, t6567, t6568, t6569, t6571, t6572)
}
