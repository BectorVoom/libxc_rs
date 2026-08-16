//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta651(t4194: f64, t5398: f64, t607: f64, t750: f64, t32: f64, t5519: f64, t2517: f64, t707: f64, t16616: f64, t2535: f64, t16701: f64, t2427: f64, t13133: f64, t4101: f64, t2371: f64, t17083: f64, t225: f64, t16805: f64, t68: f64, t16752: f64, t252: f64, t13396: f64, t1499: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57965, t57973, t57992, t58021, t58047) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2192(t4194, t5398, t607, t750, t32, t5519, t2517, t707, t16616, t2535, t16701, t2427);
        let (t58052, t58057, t58143, t58181, t58262, t58313) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2193(t13133, t4101, t16616, t2371, t17083, t225, t16805, t68, t16752, t252, t13396, t1499);
    (t57965, t57973, t57992, t58021, t58047, t58052, t58057, t58143, t58181, t58262, t58313)
}
