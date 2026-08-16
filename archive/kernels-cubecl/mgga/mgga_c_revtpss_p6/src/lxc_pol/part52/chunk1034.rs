//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1034/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1034<F: Float>(t1459: F, t8614: F, t116: F, t8460: F, t670: F, t572: F, t2089: F, t7002: F, t651: F, t8686: F, t8626: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32372 = t1459 * t8614;
    let t32373 = F::cast_from(3.0_f64) * t32372;
    let t32374 = t116 * t8460;
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = F::cast_from(6.0_f64) * t32376;
    let t32385 = t2089 * t7002;
    let t32386 = t651 * t32385;
    let t32387 = t8686 * t670;
    let t32388 = t651 * t32387;
    let t32389 = t8626 * t116;
    (t32373, t32374, t32375, t32377, t32385, t32386, t32387, t32388, t32389)
}
