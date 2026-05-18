//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1132/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1132<F: Float>(t1089: F, t33304: F, t3322: F, t33494: F, t3330: F, t33312: F, t11808: F, t30187: F, t3131: F, t5658: F, t1084: F, t29568: F) -> (F, F, F, F, F, F) {
    let t34050 = t33304 * t1089;
    let t34052 = t33494 * t3322;
    let t34054 = t33312 * t3330;
    let t34056 = t11808 * t30187;
    let t34058 = t3131 * t5658;
    let t34060 = t1084 * t34058 * t29568;
    (t34050, t34052, t34054, t34056, t34058, t34060)
}
