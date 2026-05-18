//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 928/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk928<F: Float>(t10698: F, t3296: F, t2124: F, t5168: F, t3295: F, t2201: F, t3324: F, t3336: F, t2096: F, t547: F) -> (F, F, F, F, F, F) {
    let t10699 = t10698 * t3296;
    let t10700 = F::new(0.12805040077930161442e0) * t10699;
    let t10701 = t2124 * t5168;
    let t10702 = t3295 * t10701;
    let t10705 = t2201 * t3336 * t3324;
    let t10707 = t547 * t2096;
    (t10699, t10700, t10701, t10702, t10705, t10707)
}
