//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 885/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk885<F: Float>(t6817: F, t969: F, t3034: F, t6423: F, t1219: F, t6789: F, t1831: F, t5233: F, t6808: F, t6805: F, t4758: F, t5253: F, t6406: F, t9634: F, t3577: F, t6804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20475 = t6817 * t969;
    let t20478 = t6423 * t3034;
    let t20479 = t20478 * t969;
    let t20486 = t6789 * t1219;
    let t20489 = t1831 * t5233;
    let t20492 = t6808 * t1219;
    let t20495 = t6805 * t1219;
    let t20498 = t5253 * t4758;
    let t20501 = t6406 * t9634;
    let t20502 = t20501 * t969;
    let t20505 = t6804 * t3577;
    (t20475, t20479, t20486, t20489, t20492, t20495, t20498, t20502, t20505)
}
