//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 868/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk868<F: Float>(t34029: F, t30934: F, t8597: F, t2264: F, t30797: F, t7839: F, t8518: F, t8522: F, t31699: F, t8526: F, t7637: F, t8506: F, t368: F, t4806: F, t1980: F, t7476: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34030 = 0.12862205435420921092e-2 * t34029;
    let t34031 = t30934 * t8597;
    let t34032 = 0.11321313224257494744e-1 * t34031;
    let t34033 = t30797 * t2264;
    let t34035 = t7839 * t8518;
    let t34036 = 0.21437009059034868486e-3 * t34035;
    let t34037 = t7839 * t8522;
    let t34038 = 0.21437009059034868486e-3 * t34037;
    let t34039 = t31699 * t8526;
    let t34043 = t7637 * t8506;
    let t34050 = t368 * t4806;
    let t34052 = t1980 * t7476 * t34050;
    (t34030, t34032, t34033, t34036, t34038, t34039, t34043, t34050, t34052)
}
