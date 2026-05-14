//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1036/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1036<F: Float>(t1537: F, t27571: F, t14733: F, t14752: F, t14804: F, t1529: F, t1538: F, t1542: F, t21759: F, t2293: F, t25665: F, t25667: F, t25683: F, t27511: F, t27516: F, t4431: F, t6518: F, t6541: F, t8350: F, t8366: F, t8369: F, t8381: F) -> (F,) {
    let t27572 = t27571 * t1537;
    let t27577 = -t25665 - t25667 + 0.58482233974552040708e0 * t1542 * t27511 + 0.17315755899375863299e2 * t14733 * t8381 + 1.0 * t27516 * t1538 + 2.0 * t21759 * t2293 + 2.0 * t6518 * t6541 - 2.0 * t14804 * t8350 + 1.0 * t4431 * t8366 + 1.0 * t1529 * t27572 + 0.32164683177870697974e2 * t14752 * t8369 - t25683;
    (t27577,)
}
