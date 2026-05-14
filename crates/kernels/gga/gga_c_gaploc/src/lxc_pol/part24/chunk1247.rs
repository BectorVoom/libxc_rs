//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1247/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1247<F: Float>(t10552: F, t6974: F, t10608: F, t6907: F, t9272: F, t1445: F, t1562: F, t31124: F, t31127: F, t31130: F, t31132: F, t31135: F, t31145: F, t31719: F, t34900: F, t34903: F, t34905: F, t34910: F, t34912: F, t34914: F, t34917: F) -> (F,) {
    let t34919 = 0.92023022289409799224e1 * t6974 * t10552;
    let t34921 = t9272 * t10608 * t6907;
    let t34922 = 0.51762950037793012063e1 * t34921;
    let t34923 = t34900 + t34903 - t34905 - 0.62115540045351614476e2 * t1562 * t1445 * t31719 - t34910 + t34912 + t34914 + t34917 + t34919 - t34922 + t31124 - t31127 + t31130 + t31132 + t31135 + t31145;
    (t34923,)
}
