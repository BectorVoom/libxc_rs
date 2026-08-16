//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1831/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1831<F: Float>(t1424: F, t1903: F, t23042: F, t4076: F, t47504: F, t47863: F, t47904: F, t73641: F, t73656: F, t73662: F, t73666: F, t73673: F, t73707: F, t73712: F, t85480: F, t85484: F, t85509: F, t86285: F, t86296: F) -> F {
    let t92248 = F::cast_from(0.23417857294518679245e0_f64) * t85480 + F::cast_from(0.23417857294518679245e0_f64) * t85484 + F::cast_from(0.39029762157531132076e-2_f64) * t73641 + F::cast_from(0.12142592671231907757e0_f64) * t47863 + F::cast_from(0.69394917116090352835e-2_f64) * t73656 + F::cast_from(0.78059524315062264152e-1_f64) * t73662 - F::cast_from(0.1561190486301245283e0_f64) * t73666 + t47504 - F::cast_from(0.43902994552903410657e-1_f64) * t73673 + F::cast_from(0.52683593463484092788e1_f64) * t1424 * t4076 * t1903 * t23042 - F::cast_from(0.21951497276451705328e-1_f64) * t85509 - F::cast_from(0.11708928647259339623e0_f64) * t86285 - F::cast_from(0.12142592671231907757e0_f64) * t47904 + F::cast_from(0.87805989105806821314e-1_f64) * t73707 - F::cast_from(0.69394917116090352835e-2_f64) * t73712 - F::cast_from(0.11708928647259339623e0_f64) * t86296;
    t92248
}
