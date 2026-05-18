//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 696/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk696<F: Float>(t1245: F, t914: F, t1250: F, t2439: F, t3246: F, t3259: F, t3260: F, t3266: F, t3269: F, t3270: F, t397: F, t943: F, t946: F) -> (F, F) {
    let t3273 = t914 * t1245;
    let t3278 = F::new(0.13170898365871023197e1) * t3259 * t3260 + F::new(0.65854491829355115987e0) * t2439 * t1250 + F::new(0.65854491829355115987e0) * t943 * t3266 - F::new(0.65854491829355115987e0) * t3269 * t3270 + F::new(0.65854491829355115987e0) * t3273 * t946 + F::new(0.65854491829355115987e0) * t397 * t3246;
    (t3273, t3278)
}
