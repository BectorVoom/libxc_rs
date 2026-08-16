//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2001;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta450(t1706: f64, t3545: f64, t11818: f64, t1735: f64, t248: f64, t1213: f64, t11789: f64, t1653: f64, t1227: f64, t15437: f64, t3505: f64, t3576: f64, t5064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15727, t15730, t15731, t15734, t15735, t15737) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2001(t1706, t3545, t11818, t1735, t248, t1213, t11789, t1653, t1227, t15437, t3505);
        let t15740 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2002(t3576, t5064);
    (t15727, t15730, t15731, t15734, t15735, t15737, t15740)
}
