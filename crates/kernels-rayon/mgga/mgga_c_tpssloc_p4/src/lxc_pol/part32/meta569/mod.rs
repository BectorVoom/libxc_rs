//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1940;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta569(t28378: f64, t28405: f64, t235: f64, t5612: f64, t6657: f64, t5617: f64, t23008: f64, t5585: f64, t16758: f64, t232: f64, t6646: f64, t1888: f64, t17030: f64, t16815: f64, t2632: f64, t22996: f64, t1909: f64, t226: f64, t23174: f64, t25310: f64, t26613: f64, t26667: f64, t26673: f64, t5575: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28406, t28407, t28409, t28411, t28413, t28418, t28419, t28420) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1940(t28378, t28405, t235, t5612, t6657, t5617, t23008, t5585, t16758, t232, t6646, t1888);
        let (t28422, t28423, t28426, t28427, t28430) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1941(t17030, t232, t6646, t1888, t16815, t2632, t22996, t1909, t226, t23174, t25310, t26613, t26667, t26673, t28407, t28409, t28411, t28413, t28420, t5575, t812);
    (t28406, t28407, t28409, t28411, t28413, t28418, t28419, t28422, t28423, t28426, t28427, t28430)
}
