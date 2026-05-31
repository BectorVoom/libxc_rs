//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 990/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk990<F: Float>(t7447: F, t8813: F, t8817: F, t7440: F, t8820: F, t2274: F, t30307: F, t1181: F, t23688: F, t599: F, t7346: F, t7433: F, t8966: F) -> (F, F, F, F, F, F) {
    let t35070 = t7447 * t8813;
    let t35071 = F::cast_from(0.84046875e-1_f64) * t35070;
    let t35072 = t7447 * t8817;
    let t35073 = F::cast_from(0.84046875e-1_f64) * t35072;
    let t35074 = t7440 * t8820;
    let t35075 = F::cast_from(0.5603125e-1_f64) * t35074;
    let t35076 = t30307 * t2274;
    let t35088 = t7346 * t1181 * t599 * t23688;
    let t35089 = F::cast_from(0.21437009059034868486e-3_f64) * t35088;
    let t35090 = t7433 * t8966;
    (t35071, t35073, t35075, t35076, t35089, t35090)
}
