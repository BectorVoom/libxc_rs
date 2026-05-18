//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1070/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1070<F: Float>(t1411: F, t1983: F, t7585: F, t7586: F, t1165: F, t4555: F, t604: F, t7493: F, t142: F, t5170: F, t8888: F, t5164: F, t8806: F) -> (F, F, F, F) {
    let t35051 = t7585 * t7586 * t1983 * t1411;
    let t35055 = t7493 * t1165 * t604 * t4555;
    let t35059 = t8888 * t142 * t5170;
    let t35062 = t8806 * t142 * t5164;
    (t35051, t35055, t35059, t35062)
}
