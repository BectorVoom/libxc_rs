//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 957/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk957<F: Float>(t11386: F, t2437: F, t13434: F, t18651: F, t11413: F, t1445: F, t2293: F, t4527: F, t13276: F, t4540: F, t4673: F, t13438: F, t4953: F) -> (F, F, F, F, F) {
    let t46212 = F::new(0.35750489951850426669e0) * t2437 * t11386;
    let t46216 = F::new(0.27606906686822939767e2) * t18651 * t13434;
    let t46220 = F::new(0.27606906686822939767e2) * t4527 * t1445 * t11413 * t2293;
    let t46223 = F::new(0.14300195980740170667e1) * t4540 * t4673 * t13276;
    let t46225 = F::new(0.69017266717057349418e1) * t4953 * t13438;
    (t46212, t46216, t46220, t46223, t46225)
}
