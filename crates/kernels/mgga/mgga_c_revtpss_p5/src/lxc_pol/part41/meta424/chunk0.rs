//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1482/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1482<F: Float>(t10227: F, t96: F, t10199: F, t2175: F, t2289: F, t8264: F, t31377: F, t571: F, t1464: F, t8372: F, t31027: F, t31271: F) -> (F, F, F, F, F, F) {
    let t116946 = t96 * t10227;
    let t116968 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t10199 * t2175;
    let t116969 = t2289 * t8264;
    let t117369 = F::cast_from(2.0_f64) * t571 * t31377;
    let t117374 = F::cast_from(2.0_f64) * t8372 * t1464;
    let t117450 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t31027 * t31271;
    (t116946, t116968, t116969, t117369, t117374, t117450)
}
