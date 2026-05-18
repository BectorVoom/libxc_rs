//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 570/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk570<F: Float>(t3211: F, t779: F, t3276: F, t740: F, t3234: F, t795: F, t835: F, t723: F, t2580: F, t2089: F, t3209: F, t7226: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9719 = t779 * t3211;
    let t9722 = t3276 * t740;
    let t9725 = t795 * t3234;
    let t9726 = t9725 * t740;
    let t9729 = t835 * t3234;
    let t9730 = t9729 * t723;
    let t9731 = t2580 * t9730;
    let t9734 = t2089 * t3209;
    let t9735 = t9734 * t723;
    let t9736 = t7226 * t9735;
    (t9719, t9722, t9725, t9726, t9729, t9730, t9731, t9734, t9735, t9736)
}
