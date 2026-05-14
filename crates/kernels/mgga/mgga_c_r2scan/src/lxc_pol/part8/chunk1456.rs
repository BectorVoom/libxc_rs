//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1456/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1456<F: Float>(t322: F, t1018: F, t10484: F, t10486: F, t10489: F, t2405: F, t2951: F, t2954: F, t330: F, t35109: F, t35164: F, t35205: F, t35249: F, t837: F, t838: F, t9698: F, t18990: F, t18995: F, t23741: F, t23752: F, t32133: F, t32134: F, t32138: F, t32139: F, t32143: F, t32146: F, t374: F) -> (F,) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t35250 = piecewise5(t323, 3.0 * t1018 * t330 * t9698 + t10484 * t330 * t837 + t10489 * t330 * t837 + 3.0 * t2405 * t2951 * t330 + 3.0 * t10486 * t838 + 3.0 * t2405 * t2954 + t330 * t35109, t331, t35164 + t35205, t35249);
    let t35252 = t35250 * t374 - t18990 + t18995 - t23741 + t23752 - t32133 - t32134 + t32138 + t32139 + t32143 + t32146;
    (t35252,)
}
