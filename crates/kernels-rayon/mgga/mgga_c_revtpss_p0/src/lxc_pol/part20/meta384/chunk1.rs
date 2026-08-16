//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1403/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1403(t10997: f64, t41066: f64, t11007: f64, t252: f64, t786: f64, t11009: f64, t123: f64, t676: f64, t41026: f64, t41029: f64, t41032: f64, t41034: f64, t41037: f64, t41038: f64, t41043: f64, t41049: f64, t41052: f64, t41056: f64, t41058: f64, t41060: f64, t41063: f64) -> f64 {
    let t41067 = t41066 * t10997;
    let t41070 = t786 * t252 * t11007;
    let t41073 = t41070 * t123 * t676 * t11009;
    let t41075 = 0.39029762157531132076e-1_f64 * t41026 + 0.69394917116090352835e-2_f64 * t41029 - 0.13170898365871023197e0_f64 * t41032 + 0.1040793657534163522e-1_f64 * t41034 + t41037 + 0.15611904863012452831e0_f64 * t41038 + 0.23417857294518679245e0_f64 * t41043 + t41049 - 0.1561190486301245283e0_f64 * t41052 - 0.69394917116090352835e-2_f64 * t41056 - 0.11708928647259339623e0_f64 * t41058 + 0.12142592671231907757e0_f64 * t41060 + 0.13170898365871023197e0_f64 * t41063 + 0.23417857294518679245e0_f64 * t41067 - 0.23417857294518679245e0_f64 * t41073;
    t41075
}
