//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1849;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta518(t1824: f64, t2006: f64, t1352: f64, t6914: f64, t7737: f64, t1351: f64, t1834: f64, t550: f64, t6976: f64, t1992: f64, t3807: f64, t5335: f64, t22633: f64, t5345: f64, t1799: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26403, t26404, t26406, t26410, t26411, t26412, t26414) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1849(t1824, t2006, t1352, t6914, t7737, t1351, t1834, t550, t6976, t1992, t3807, t5335);
        let (t26415, t26416, t26418, t26419, t26421) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1850(t26414, t6976, t22633, t5345, t1992, t1799, t562);
    (t26403, t26404, t26406, t26410, t26411, t26412, t26414, t26415, t26416, t26418, t26419, t26421)
}
