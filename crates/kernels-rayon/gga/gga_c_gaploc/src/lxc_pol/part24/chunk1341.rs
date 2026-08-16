//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1341/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1341(t33936: f64, t20671: f64, t24505: f64, t28069: f64, t10783: f64, t1457: f64, t2004: f64, t32173: f64, t32215: f64, t33905: f64, t33907: f64, t33912: f64, t33916: f64, t33920: f64, t33922: f64, t33927: f64, t33929: f64, t33932: f64, t33933: f64, t33934: f64, t4614: f64, t4820: f64, t7513: f64, t833: f64) -> f64 {
    let t33937 = 0.38342925953920749676e0_f64 * t33936;
    let t33942 = t28069 * t20671 * t24505;
    let t33943 = 0.42603251059911944084e0_f64 * t33942;
    let t33944 = t33905 + t33907 + 0.30674340763136599741e2_f64 * t833 * t4614 * t10783 - t33912 - t33916 - t33920 + t33922 + 0.35750489951850426669e0_f64 * t2004 * t1457 * t32173 + t33927 + t33929 - t33932 - t33933 - t33934 + t33937 - 0.15889106645266856297e0_f64 * t7513 * t4820 * t32215 - t33943;
    t33944
}
