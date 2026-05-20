//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1042/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1042<F: Float>(t10495: F, t10498: F, t10501: F, t10503: F, t10507: F, t10511: F, t10513: F, t10978: F, t10984: F, t10987: F, t10989: F, t10992: F, t10998: F, t11000: F, t865: F, t887: F) -> F {
    let t11002 = F::cast_from(0.39512695097613069591e1_f64) * t865 * t10495 + F::cast_from(0.21951497276451705329e-1_f64) * t10498 + t10501 - t10503 - F::cast_from(0.34697458558045176417e-2_f64) * t10507 + F::cast_from(0.39029762157531132076e-1_f64) * t10511 - F::cast_from(0.19756347548806534796e1_f64) * t10513 * t887 - F::cast_from(0.65854491829355115987e0_f64) * t865 * t10978 + t10984 - t10987 + F::cast_from(0.16463622957338778996e-1_f64) * t10989 + F::cast_from(0.32927245914677557992e-1_f64) * t10992 + F::cast_from(0.58544643236296698113e-1_f64) * t10998 - F::cast_from(0.21951497276451705329e-1_f64) * t11000;
    t11002
}
