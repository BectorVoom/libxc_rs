//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 941/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk941<F: Float>(t1181: F, t33735: F, t599: F, t7413: F, t1983: F, t30127: F, t7586: F, t8791: F, t21143: F, t604: F, t7493: F, t22401: F, t1165: F, t20775: F, t30698: F, t22710: F) -> (F, F, F, F, F, F) {
    let t34953 = t7413 * t1181 * t599 * t33735;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34965 = t7413 * t1181 * t604 * t22401;
    let t34969 = t30698 * t1165 * t604 * t20775;
    let t34973 = t7413 * t1181 * t604 * t22710;
    (t34953, t34957, t34961, t34965, t34969, t34973)
}
