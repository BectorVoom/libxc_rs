//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1066/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1066<F: Float>(t1181: F, t20992: F, t7351: F, t7426: F, t20138: F, t599: F, t7413: F, t33735: F, t1983: F, t30127: F, t7586: F, t8791: F) -> (F, F, F, F) {
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34949 = t7413 * t1181 * t599 * t20138;
    let t34953 = t7413 * t1181 * t599 * t33735;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    (t34945, t34949, t34953, t34957)
}
