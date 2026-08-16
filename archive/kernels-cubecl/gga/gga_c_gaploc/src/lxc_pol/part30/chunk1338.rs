//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1338/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1338<F: Float>(t33936: F, t20671: F, t24505: F, t28069: F, t10783: F, t1457: F, t2004: F, t32173: F, t32215: F, t33905: F, t33907: F, t33912: F, t33916: F, t33920: F, t33922: F, t33927: F, t33929: F, t33932: F, t33933: F, t33934: F, t4614: F, t4820: F, t7513: F, t833: F) -> F {
    let t33937 = F::cast_from(0.38342925953920749676e0_f64) * t33936;
    let t33942 = t28069 * t20671 * t24505;
    let t33943 = F::cast_from(0.42603251059911944084e0_f64) * t33942;
    let t33944 = t33905 + t33907 + F::cast_from(0.30674340763136599741e2_f64) * t833 * t4614 * t10783 - t33912 - t33916 - t33920 + t33922 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t1457 * t32173 + t33927 + t33929 - t33932 - t33933 - t33934 + t33937 - F::cast_from(0.15889106645266856297e0_f64) * t7513 * t4820 * t32215 - t33943;
    t33944
}
