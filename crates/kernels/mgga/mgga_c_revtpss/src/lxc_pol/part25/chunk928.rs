//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 928/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk928<F: Float>(t10495: F, t10498: F, t10501: F, t10503: F, t10507: F, t10511: F, t10513: F, t10978: F, t10984: F, t10987: F, t10989: F, t10992: F, t10998: F, t11000: F, t865: F, t887: F) -> F {
    let t11002 = F::new(0.39512695097613069591e1) * t865 * t10495 + F::new(0.21951497276451705329e-1) * t10498 + t10501 - t10503 - F::new(0.34697458558045176417e-2) * t10507 + F::new(0.39029762157531132076e-1) * t10511 - F::new(0.19756347548806534796e1) * t10513 * t887 - F::new(0.65854491829355115987e0) * t865 * t10978 + t10984 - t10987 + F::new(0.16463622957338778996e-1) * t10989 + F::new(0.32927245914677557992e-1) * t10992 + F::new(0.58544643236296698113e-1) * t10998 - F::new(0.21951497276451705329e-1) * t11000;
    t11002
}
