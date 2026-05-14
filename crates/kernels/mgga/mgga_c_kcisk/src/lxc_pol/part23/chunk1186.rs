//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1186/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1186<F: Float>(t32592: F, t9379: F, t9365: F, t267: F, t3042: F, t1101: F, t2697: F, t119: F, t3391: F, t1110: F, t918: F, t3368: F, t9375: F, t15484: F, t2694: F, t9371: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32593 = t9379 * t32592;
    let t32595 = t9365 * t32592;
    let t32597 = t267 * t3042;
    let t32599 = t1101 * t32597 * t2697;
    let t32601 = t3391 * t119;
    let t32603 = t1101 * t32601 * t2697;
    let t32605 = t1110 * t918;
    let t32607 = t1101 * t32605 * t2697;
    let t32610 = t3368 * t9375 * t2697;
    let t32613 = t15484 * t2694 * t2697;
    let t32616 = t3368 * t9371 * t2697;
    (t32593, t32595, t32597, t32599, t32601, t32603, t32605, t32607, t32610, t32613, t32616)
}
