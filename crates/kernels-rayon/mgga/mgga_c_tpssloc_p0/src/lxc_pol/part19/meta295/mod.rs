//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta295(t649: f64, t671: f64, t157: f64, t9929: f64, t2379: f64, t262: f64, t9897: f64, t2570: f64, t67: f64, t792: f64, t131: f64, t9558: f64, t205: f64, t4126: f64, t782: f64, t68: f64, t822: f64, t2644: f64, t820: f64, t2617: f64, t4177: f64, t2628: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12734, t12908, t12935, t12939, t12998, t13004) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1076(t649, t671, t157, t9929, t2379, t262, t9897, t2570, t67, t792, t131, t9558);
        let (t13005, t13012, t13151, t13222, t13254, t13257) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1077(t13004, t205, t4126, t782, t68, t822, t2644, t820, t2617, t4177, t2628, t836);
    (t12734, t12908, t12935, t12939, t12998, t13005, t13012, t13151, t13222, t13254, t13257)
}
