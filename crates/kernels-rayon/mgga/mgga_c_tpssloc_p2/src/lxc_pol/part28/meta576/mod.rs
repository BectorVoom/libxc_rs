//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta576(t23097: f64, t4234: f64, t776: f64, t815: f64, t13176: f64, t6620: f64, t849: f64, t25097: f64, t81782: f64, t81783: f64, t1516: f64, t81769: f64, t23133: f64, t4261: f64, t25111: f64, t25115: f64, t87229: f64, t23132: f64, t4166: f64, t25068: f64, t2707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87316, t87322, t87328, t87330) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1858(t23097, t4234, t776, t815, t13176, t6620, t849, t25097, t81782, t81783, t1516, t81769);
        let (t87332, t87335, t87338, t87341, t87343) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1859(t23133, t4261, t25111, t81782, t81783, t25115, t87229, t23132, t4166, t849, t25068, t2707);
    (t87316, t87322, t87328, t87330, t87332, t87335, t87338, t87341, t87343)
}
