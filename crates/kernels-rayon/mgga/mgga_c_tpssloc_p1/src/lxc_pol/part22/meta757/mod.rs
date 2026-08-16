//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta757(t1102: f64, t21785: f64, t43889: f64, t18746: f64, t4756: f64, t14813: f64, t5999: f64, t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64, t18730: f64, t4764: f64, t21801: f64, t699: f64, t21788: f64, t21791: f64, t1113: f64, t136: f64, t71177: f64, t3297: f64, t71181: f64, t71185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71461, t71463, t71465, t71467) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2542(t1102, t21785, t43889, t18746, t4756, t14813, t5999, t71183, t71187, t71446, t71449, t71452, t71454, t71456, t71458);
        let (t71468, t71470, t71472, t71474, t71477, t71480, t71483) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2543(t18730, t4764, t21801, t699, t21788, t21791, t1113, t136, t71177, t3297, t71181, t71185);
    (t71461, t71463, t71465, t71467, t71468, t71470, t71472, t71474, t71477, t71480, t71483)
}
