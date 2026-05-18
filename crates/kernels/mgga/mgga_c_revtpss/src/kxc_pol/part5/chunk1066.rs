//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1066/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1066<F: Float>(t1385: F, t5710: F, t1904: F, t3899: F, t689: F, t3920: F, t5603: F, t2435: F, t5718: F, t1893: F, t2453: F, t3908: F) -> (F, F, F, F, F) {
    let t14255 = t1385 * t5710;
    let t14274 = t3899 * t1904;
    let t14276 = F::new(0.10975748638225852664e-1) * t689 * t14274;
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    (t14255, t14276, t14280, t14290, t14294)
}
