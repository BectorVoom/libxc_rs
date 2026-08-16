//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1403/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1403<F: Float>(t10997: F, t41066: F, t11007: F, t252: F, t786: F, t11009: F, t123: F, t676: F, t41026: F, t41029: F, t41032: F, t41034: F, t41037: F, t41038: F, t41043: F, t41049: F, t41052: F, t41056: F, t41058: F, t41060: F, t41063: F) -> F {
    let t41067 = t41066 * t10997;
    let t41070 = t786 * t252 * t11007;
    let t41073 = t41070 * t123 * t676 * t11009;
    let t41075 = F::cast_from(0.39029762157531132076e-1_f64) * t41026 + F::cast_from(0.69394917116090352835e-2_f64) * t41029 - F::cast_from(0.13170898365871023197e0_f64) * t41032 + F::cast_from(0.1040793657534163522e-1_f64) * t41034 + t41037 + F::cast_from(0.15611904863012452831e0_f64) * t41038 + F::cast_from(0.23417857294518679245e0_f64) * t41043 + t41049 - F::cast_from(0.1561190486301245283e0_f64) * t41052 - F::cast_from(0.69394917116090352835e-2_f64) * t41056 - F::cast_from(0.11708928647259339623e0_f64) * t41058 + F::cast_from(0.12142592671231907757e0_f64) * t41060 + F::cast_from(0.13170898365871023197e0_f64) * t41063 + F::cast_from(0.23417857294518679245e0_f64) * t41067 - F::cast_from(0.23417857294518679245e0_f64) * t41073;
    t41075
}
