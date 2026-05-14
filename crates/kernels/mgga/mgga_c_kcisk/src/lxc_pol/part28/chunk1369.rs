//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1369/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1369<F: Float>(t116737: F, t116738: F, t116741: F, t116745: F, t116748: F, t116768: F, t116771: F, t116960: F, t121044: F, t121546: F, t1693: F, t1763: F, t20: F, t2785: F, t32921: F, t32948: F, t33021: F, t33056: F, t34018: F, t35136: F, t35191: F, t4830: F, t68510: F, t7261: F, t8831: F, t9664: F) -> (F,) {
    let t121578 = -0.26805555555555555556e-2 * t33056 * t121546 + 0.16083333333333333334e-1 * t33056 * t121044 - t116737 - 0.58958024691358024689e-2 * t116738 + t116741 + 0.46296296296296296297e-2 * t116745 - t116748 - 0.41666666666666666668e-1 * t9664 * t7261 * t33021 * t68510 - 0.92592592592592592594e-2 * t116960 * t34018 + t116768 + t116771 - 0.10185185185185185186e0 * t4830 * t35191 * t2785 - 0.10185185185185185186e0 * t1693 * t1763 * t8831 * t20 * t2785 + 0.40208333333333333335e-2 * t32948 * t35136 + 0.40208333333333333335e-2 * t32921 * t35136;
    (t121578,)
}
