//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1039/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1039<F: Float>(t3131: F, t5658: F, t1084: F, t29568: F, t11781: F, t3368: F, t11892: F, t11473: F, t3322: F, t3363: F, t3330: F, t33560: F, t1971: F, t8785: F, t29571: F, t1461: F, t8709: F) -> (F, F, F, F, F, F, F, F, F) {
    let t34058 = t3131 * t5658;
    let t34060 = t1084 * t34058 * t29568;
    let t34062 = t11781 * t3368;
    let t34066 = t11892 * t3368;
    let t34069 = t3363 * t11473 * t3322;
    let t34071 = t33560 * t3330;
    let t34073 = t1971 * t8785;
    let t34075 = t1084 * t34073 * t29571;
    let t34077 = t1461 * t8709;
    (t34058, t34060, t34062, t34066, t34069, t34071, t34073, t34075, t34077)
}
