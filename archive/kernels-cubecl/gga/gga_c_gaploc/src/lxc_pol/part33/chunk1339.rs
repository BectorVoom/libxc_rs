//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1339/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1339<F: Float>(t34913: F, t34407: F, t6716: F, t6717: F, t10552: F, t6974: F, t10608: F, t6907: F, t9272: F, t10466: F, t7014: F, t20843: F, t2487: F, t3395: F) -> (F, F, F, F, F, F) {
    let t34914 = F::cast_from(0.59584149919750711116e-1_f64) * t34913;
    let t34917 = F::cast_from(0.13803453343411469884e2_f64) * t6716 * t6717 * t34407;
    let t34919 = F::cast_from(0.92023022289409799224e1_f64) * t6974 * t10552;
    let t34921 = t9272 * t10608 * t6907;
    let t34922 = F::cast_from(0.51762950037793012063e1_f64) * t34921;
    let t34927 = t7014 * t10466;
    let t34928 = F::cast_from(0.51123901271894332902e0_f64) * t34927;
    let t34930 = t2487 * t20843 * t3395;
    (t34914, t34917, t34919, t34922, t34928, t34930)
}
