//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1279/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1279<F: Float>(t107: F, t12223: F, t12250: F, t1445: F, t1710: F, t2021: F, t2023: F, t33901: F, t33905: F, t33907: F, t33912: F, t33916: F, t33920: F, t33922: F, t33927: F, t33929: F, t33932: F, t33933: F, t33934: F, t33937: F, t33943: F, t813: F) -> (F,) {
    let t39330 = -0.46011511144704899612e1 * t813 * t1445 * t12223 * t1710 - t33901 + t33905 + t33907 - t33912 - t33916 - t33920 + t33922 + t33927 + t33929 - t33932 - t33933 - t33934 + t33937 - t33943 + 0.79445533226334281486e-1 * t2021 * t12250 * t107 * t2023;
    (t39330,)
}
