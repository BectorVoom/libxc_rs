//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2161/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2161<F: Float>(t15609: F, t16432: F, t15604: F, t1089: F, t1668: F, t3259: F, t15780: F, t4983: F, t3075: F, t5004: F, t359: F, t4930: F) -> (F, F, F, F, F, F) {
    let t16433 = t16432 * t15609;
    let t16436 = t16432 * t15604;
    let t16440 = t3259 * t1668 * t1089;
    let t16443 = t15780 * t4983;
    let t16446 = t5004 * t3075;
    let t16449 = t359 * t4930;
    (t16433, t16436, t16440, t16443, t16446, t16449)
}
