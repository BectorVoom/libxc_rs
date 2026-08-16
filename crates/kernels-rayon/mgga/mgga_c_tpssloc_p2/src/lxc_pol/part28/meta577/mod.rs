//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta577(t1516: f64, t81763: f64, t23083: f64, t25094: f64, t1510: f64, t2379: f64, t25119: f64, t815: f64, t2631: f64, t47285: f64, t6605: f64, t9972: f64, t12971: f64, t1894: f64, t236: f64, t6591: f64, t23046: f64, t4184: f64, t812: f64, t836: f64, t13080: f64, t23146: f64, t242: f64, t81816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87345, t87347, t87351, t87355) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860(t1516, t81763, t23083, t25094, t1510, t2379, t25119, t815, t2631, t47285, t6605, t9972);
        let (t87359, t87363, t87365, t87368) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1861(t12971, t1894, t236, t6591, t23046, t4184, t812, t836, t13080, t23146, t242, t81816);
    (t87345, t87347, t87351, t87355, t87359, t87363, t87365, t87368)
}
