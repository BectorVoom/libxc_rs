//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1831/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1831(t1424: f64, t1903: f64, t23042: f64, t4076: f64, t47504: f64, t47863: f64, t47904: f64, t73641: f64, t73656: f64, t73662: f64, t73666: f64, t73673: f64, t73707: f64, t73712: f64, t85480: f64, t85484: f64, t85509: f64, t86285: f64, t86296: f64) -> f64 {
    let t92248 = 0.23417857294518679245e0_f64 * t85480 + 0.23417857294518679245e0_f64 * t85484 + 0.39029762157531132076e-2_f64 * t73641 + 0.12142592671231907757e0_f64 * t47863 + 0.69394917116090352835e-2_f64 * t73656 + 0.78059524315062264152e-1_f64 * t73662 - 0.1561190486301245283e0_f64 * t73666 + t47504 - 0.43902994552903410657e-1_f64 * t73673 + 0.52683593463484092788e1_f64 * t1424 * t4076 * t1903 * t23042 - 0.21951497276451705328e-1_f64 * t85509 - 0.11708928647259339623e0_f64 * t86285 - 0.12142592671231907757e0_f64 * t47904 + 0.87805989105806821314e-1_f64 * t73707 - 0.69394917116090352835e-2_f64 * t73712 - 0.11708928647259339623e0_f64 * t86296;
    t92248
}
