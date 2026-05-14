//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 544/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk544<F: Float>(t1211: F, t5230: F, t1294: F, t1774: F, t1277: F, t3358: F, t3579: F, t5044: F, t5049: F, t5054: F, t5058: F, t1209: F, t1811: F, t1256: F, t1804: F, t1786: F) -> (F, F, F, F, F, F, F) {
    let t5231 = t1211 * t5230;
    let t5236 = t1774 * t1294;
    let t5237 = t1277 * t5236;
    let t5245 = t3579 - 0.4938888888888888889e-2 * t3358 - 0.4938888888888888889e-2 * t5044 - 0.9877777777777777778e-2 * t5049 + 0.29633333333333333334e-1 * t5054 + 0.14816666666666666667e-1 * t5058;
    let t5246 = t1211 * t5245;
    let t5251 = t1209 * t1811;
    let t5254 = t1804 * t1256;
    let t5256 = t1786 * t1256;
    (t5231, t5237, t5245, t5246, t5251, t5254, t5256)
}
