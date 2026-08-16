//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1411/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1411(t34913: f64, t34407: f64, t6716: f64, t6717: f64, t10552: f64, t6974: f64, t10608: f64, t6907: f64, t9272: f64, t1445: f64, t1562: f64, t31124: f64, t31127: f64, t31130: f64, t31132: f64, t31135: f64, t31145: f64, t31719: f64, t34900: f64, t34903: f64, t34905: f64, t34910: f64, t34912: f64) -> f64 {
    let t34914 = 0.59584149919750711116e-1_f64 * t34913;
    let t34917 = 0.13803453343411469884e2_f64 * t6716 * t6717 * t34407;
    let t34919 = 0.92023022289409799224e1_f64 * t6974 * t10552;
    let t34921 = t9272 * t10608 * t6907;
    let t34922 = 0.51762950037793012063e1_f64 * t34921;
    let t34923 = t34900 + t34903 - t34905 - 0.62115540045351614476e2_f64 * t1562 * t1445 * t31719 - t34910 + t34912 + t34914 + t34917 + t34919 - t34922 + t31124 - t31127 + t31130 + t31132 + t31135 + t31145;
    t34923
}
