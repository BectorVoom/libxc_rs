//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1365/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1365<F: Float>(t17056: F, t1869: F, t34160: F, t654: F, t17182: F, t35096: F, t9664: F, t35158: F, t4998: F, t1333: F, t35233: F, t35107: F, t33002: F, t1900: F, t415: F, t8666: F) -> (F, F, F, F, F, F, F, F) {
    let t121440 = t1869 * t17056 * t654 * t34160;
    let t121442 = t17182 * t35096;
    let t121443 = t9664 * t121442;
    let t121446 = t9664 * t4998 * t35158;
    let t121454 = t1333 * t35233;
    let t121456 = t17182 * t35107;
    let t121457 = t33002 * t121456;
    let t121460 = t415 * t8666 * t1900;
    (t121440, t121442, t121443, t121446, t121454, t121456, t121457, t121460)
}
