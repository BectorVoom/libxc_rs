//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 732/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk732<F: Float>(t42953: F, t1841: F, t2576: F, t35435: F, t161: F, t36610: F, t42963: F, t2558: F, t36798: F, t9647: F, t10697: F, t10742: F, t11662: F, t2554: F, t7064: F, t35611: F, t5539: F) -> (F, F, F, F, F, F, F, F) {
    let t44740 = 0.17090058289204942853e-2 * t42953;
    let t44744 = 0.59815204012217299984e-2 * t1841 * t35435 * t2576;
    let t44745 = t36610 * t161;
    let t44748 = 0.25635087433807414279e-2 * t1841 * t44745 * t2576;
    let t44751 = 0.15381052460284448568e-1 * t42963;
    let t44755 = t9647 * t36798 * t2558;
    let t44756 = 0.32043859292259267849e-3 * t44755;
    let t44758 = t9647 * t10697 * t10742;
    let t44759 = 0.19226315575355560709e-2 * t44758;
    let t44761 = t7064 * t11662 * t2554;
    let t44762 = 0.32043859292259267849e-3 * t44761;
    let t44764 = t9647 * t5539 * t35611;
    (t44740, t44744, t44748, t44751, t44756, t44759, t44762, t44764)
}
