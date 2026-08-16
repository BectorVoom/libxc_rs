//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1446/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1446(t107: f64, t12223: f64, t12250: f64, t1445: f64, t1710: f64, t2021: f64, t2023: f64, t33901: f64, t33905: f64, t33907: f64, t33912: f64, t33916: f64, t33920: f64, t33922: f64, t33927: f64, t33929: f64, t33932: f64, t33933: f64, t33934: f64, t33937: f64, t33943: f64, t813: f64) -> f64 {
    let t39330 = -0.46011511144704899612e1_f64 * t813 * t1445 * t12223 * t1710 - t33901 + t33905 + t33907 - t33912 - t33916 - t33920 + t33922 + t33927 + t33929 - t33932 - t33933 - t33934 + t33937 - t33943 + 0.79445533226334281486e-1_f64 * t2021 * t12250 * t107 * t2023;
    t39330
}
