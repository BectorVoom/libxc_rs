//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1676;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta491(t26361: f64, t225: f64, t7919: f64, t2085: f64, t5210: f64, t1824: f64, t5250: f64, t1352: f64, t26393: f64, t1825: f64, t24116: f64, t26406: f64, t1336: f64, t22707: f64, t24099: f64, t26379: f64, t26381: f64, t26386: f64, t26390: f64, t26398: f64, t26412: f64, t26416: f64, t26419: f64, t26424: f64, t26427: f64, t3777: f64, t5234: f64, t5334: f64, t5344: f64, t7209: f64, t7932: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27067, t27068, t27070, t27074, t27075, t27078, t27082, t27086, t27088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1676(t26361, t225, t7919, t2085, t5210, t1824, t5250, t1352, t26393, t1825, t24116, t26406);
        let t27095 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1677(t1336, t22707, t24099, t26379, t26381, t26386, t26390, t26398, t26412, t26416, t26419, t26424, t26427, t27075, t27078, t27082, t27086, t27088, t3777, t5234, t5334, t5344, t7209, t7932);
    (t27067, t27068, t27070, t27074, t27075, t27078, t27086, t27095)
}
