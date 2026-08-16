//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3203/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3203<F: Float>(t1256: F, t24700: F, t1791: F, t21107: F, t5287: F, t70210: F, t71931: F, t71971: F, t71974: F, t71976: F, t72000: F, t72005: F, t72017: F, t84082: F) -> F {
    let t84084 = t24700 * t1256;
    let t84094 = t71931 / F::cast_from(216.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t71971 - F::cast_from(0.42874018118069736972e-3_f64) * t71974 + F::cast_from(0.14481890564325777821e-1_f64) * t84082 + F::cast_from(0.14291339372689912324e-3_f64) * t84084 - F::cast_from(0.57165357490759649295e-3_f64) * t71976 - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t72000 - F::cast_from(0.68598428988911579154e-2_f64) * t21107 * t5287 - F::cast_from(0.91464571985215438872e-2_f64) * t72005 - F::cast_from(0.42874018118069736972e-3_f64) * t72017 - F::cast_from(0.64311027177104605458e-3_f64) * t70210 * t1791;
    t84094
}
