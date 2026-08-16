//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1504;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta277(t10073: f64, t4089: f64, t10008: f64, t10015: f64, t10020: f64, t10027: f64, t10032: f64, t10035: f64, t10041: f64, t10044: f64, t10049: f64, t10062: f64, t10066: f64, t10070: f64, t1437: f64, t213: f64, t3924: f64, t4004: f64, t4087: f64, t4118: f64, t546: f64, t5745: f64, t820: f64, t9840: f64, t9891: f64, t9899: f64, t1398: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t4056: f64, t555: f64, t9990: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10074, t10076) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1504(t10073, t4089, t10008, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t10049, t10062, t10066, t10070, t1437, t213, t3924, t4004, t4087, t4118, t546, t5745, t820, t9840, t9891, t9899);
        let (t10079, t10080, t10082, t10084, t10085, t10090, t10098, t10102) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1505(t1398, t1419, t4086, t543, t2782, t4056, t555, t9990, t1432, t2470, t4107, t1433, t9288);
    (t10074, t10076, t10079, t10080, t10082, t10084, t10085, t10090, t10098, t10102)
}
