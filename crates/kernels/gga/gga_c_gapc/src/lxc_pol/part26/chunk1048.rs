//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1048/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1048<F: Float>(t1912: F, t19652: F, t3717: F, t11509: F, t5633: F, t3144: F, t34409: F, t11329: F, t8885: F, t11379: F, t11381: F, t25953: F, t28065: F, t3709: F, t11388: F, t8636: F) -> (F, F, F, F, F, F, F) {
    let t34971 = t19652 * t3717 * t1912;
    let t34973 = t11509 * t5633;
    let t34975 = t34409 * t3144;
    let t34977 = t11329 * t8885;
    let t34980 = t25953 * t11379 * t11381;
    let t34982 = t3709 * t28065;
    let t34984 = t11388 * t8636;
    (t34971, t34973, t34975, t34977, t34980, t34982, t34984)
}
