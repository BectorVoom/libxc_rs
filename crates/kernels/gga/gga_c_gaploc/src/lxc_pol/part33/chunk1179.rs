//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1179/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1179<F: Float>(t20367: F, t31775: F, t10537: F, t4379: F, t34407: F, t6716: F, t6717: F, t10552: F, t6974: F, t10608: F, t6907: F, t9272: F, t10466: F, t7014: F, t20843: F, t2487: F, t3395: F) -> (F, F, F, F, F, F, F) {
    let t34912 = 0.47667319935800568892e0 * t20367 * t31775;
    let t34913 = t4379 * t10537;
    let t34914 = 0.59584149919750711116e-1 * t34913;
    let t34917 = 0.13803453343411469884e2 * t6716 * t6717 * t34407;
    let t34919 = 0.92023022289409799224e1 * t6974 * t10552;
    let t34921 = t9272 * t10608 * t6907;
    let t34922 = 0.51762950037793012063e1 * t34921;
    let t34927 = t7014 * t10466;
    let t34928 = 0.51123901271894332902e0 * t34927;
    let t34930 = t2487 * t20843 * t3395;
    (t34912, t34914, t34917, t34919, t34922, t34928, t34930)
}
