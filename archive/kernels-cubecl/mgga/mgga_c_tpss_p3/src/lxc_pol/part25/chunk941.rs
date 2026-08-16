//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 941/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk941<F: Float>(t11521: F, t3754: F, t925: F, t140: F, t2697: F, t3749: F, t8491: F, t926: F, t242: F, t2751: F, t3758: F, t967: F) -> (F, F, F, F) {
    let t11522 = t11521 * t3754;
    let t11524 = t925 * t11522 / F::cast_from(216.0_f64);
    let t11525 = t140 * t2697;
    let t11526 = t11525 * t3749;
    let t11528 = t925 * t11526 / F::cast_from(324.0_f64);
    let t11535 = t926 * t8491;
    let t11548 = t242 * t2751 * t3758;
    let t11550 = t967 * t11548 / F::cast_from(3456.0_f64);
    (t11524, t11528, t11535, t11550)
}
