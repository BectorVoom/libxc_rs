//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1017/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1017<F: Float>(t1181: F, t19834: F, t2068: F, t599: F, t1983: F, t30262: F, t7586: F, t8406: F, t4680: F, t7346: F, t8896: F, t7433: F, t8962: F) -> (F, F, F, F) {
    let t34123 = t2068 * t1181 * t599 * t19834;
    let t34127 = t30262 * t7586 * t1983 * t8406;
    let t34130 = t7346 * t4680 * t8896;
    let t34132 = t7433 * t8962;
    (t34123, t34127, t34130, t34132)
}
