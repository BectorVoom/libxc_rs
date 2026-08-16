//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1927;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1928;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta538(t1824: f64, t2006: f64, t1352: f64, t6914: f64, t7737: f64, t1351: f64, t1834: f64, t550: f64, t6976: f64, t1992: f64, t3807: f64, t5335: f64, t22633: f64, t5345: f64, t1799: f64, t562: f64, t22705: f64, t7736: f64, t22704: f64, t6883: f64, t7741: f64, t1332: f64, t2013: f64, t22693: f64, t22707: f64, t26379: f64, t26381: f64, t26386: f64, t26390: f64, t26393: f64, t26398: f64, t26401: f64, t5230: f64, t5344: f64, t544: f64, t7747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t26403 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1926(t1824, t2006);
        let (t26404, t26406, t26410, t26411, t26412, t26414, t26415, t26416) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1927(t1352, t26403, t6914, t7737, t1351, t1834, t550, t6976, t1992, t3807, t5335, t22633);
        let (t26418, t26419, t26421) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1928(t5345, t6976, t1992, t1799, t562);
        let (t26422, t26423, t26426, t26431) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1929(t1352, t26421, t6976, t22633, t22705, t7736, t22704, t6883, t7741, t1332, t2013, t22693, t22707, t26379, t26381, t26386, t26390, t26393, t26398, t26401, t26404, t26406, t26412, t26416, t26419, t5230, t5344, t544, t7747);
    (t26403, t26404, t26410, t26411, t26414, t26415, t26418, t26421, t26422, t26423, t26426, t26431)
}
