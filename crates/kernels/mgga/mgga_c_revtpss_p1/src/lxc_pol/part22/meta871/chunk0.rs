//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3031/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3031<F: Float>(t1561: F, t40360: F, t14843: F, t40864: F, t10779: F, t14931: F, t1548: F, t2724: F, t10811: F, t14693: F, t2682: F, t2719: F, t4368: F, t820: F) -> (F, F, F, F, F) {
    let t51104 = t40360 * t1561;
    let t51106 = t40864 * t14843;
    let t51110 = t14931 * t10779 * t1548 * t2724;
    let t51112 = t10811 * t14693;
    let t51121 = t820 * t2719 * t2682 * t4368;
    (t51104, t51106, t51110, t51112, t51121)
}
