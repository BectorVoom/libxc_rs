//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1344/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1344<F: Float>(t4170: F, t6394: F, t9509: F, t41218: F, t4171: F, t9831: F, t32238: F, t48680: F, t14294: F, t9848: F, t20937: F, t32229: F, t4169: F, t9827: F, t20160: F, t33345: F) -> (F, F, F, F, F, F, F) {
    let t113557 = 4.0 * t4170 * t9509 * t6394;
    let t113563 = 24.0 * t41218 * t9831 * t4171;
    let t113565 = 6.0 * t48680 * t32238;
    let t113568 = 6.0 * t14294 * t9848 * t4171;
    let t113570 = 2.0 * t32229 * t20937;
    let t113573 = t9827 * t4169;
    let t113575 = 2.0 * t113573 * t4171;
    let t113576 = t20160 * t33345;
    (t113557, t113563, t113565, t113568, t113570, t113575, t113576)
}
