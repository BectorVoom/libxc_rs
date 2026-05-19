//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 506/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk506<F: Float>(t123: F, t7284: F, t2563: F, t9647: F, t5539: F, t7292: F, t286: F, t708: F, t9095: F, t1687: F, t9099: F, t5337: F, t5340: F, t9106: F) -> (F, F, F, F, F) {
    let t9648 = t7284 * t123;
    let t9649 = t9648 * t2563;
    let t9651 = F::cast_from(0.1922631557535556071e-2_f64) * t9647 * t9649;
    let t9652 = t5539 * t7292;
    let t9654 = F::cast_from(0.1281754371690370714e-2_f64) * t9647 * t9652;
    let t9664 = t9095 * t286 * t708;
    let t9666 = t9099 * t1687;
    let t9669 = t9106 * t5337 * t5340;
    (t9651, t9654, t9664, t9666, t9669)
}
