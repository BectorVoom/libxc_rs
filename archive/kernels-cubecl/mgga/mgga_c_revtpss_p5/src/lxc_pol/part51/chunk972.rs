//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 972/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk972<F: Float>(t32243: F, t32295: F, t532: F, t1450: F, t2014: F, t118: F, t1453: F, t32095: F, t32101: F, t32102: F, t32104: F, t32107: F, t32109: F, t32112: F, t32116: F, t32118: F, t32123: F, t32124: F, t32126: F, t32131: F, t32179: F, t32182: F, t569: F, t649: F, t8463: F, t8557: F, t8565: F) -> (F, F, F, F) {
    let t32296 = t32243 + t32295;
    let t32297 = t532 * t32296;
    let t32298 = t32297 * t1450;
    let t32299 = t2014 * t32298;
    let t32300 = -t118 * t32095 + t1453 * t8565 + t32179 * t569 - t649 * t8557 + t32101 - t32102 - F::cast_from(4.0_f64) * t32104 - t32107 - t32109 - t32112 - t32116 - t32118 - t32123 - F::cast_from(2.0_f64) * t32124 + F::cast_from(6.0_f64) * t32126 + t32131 + t32182 + t32299 - t8463;
    (t32296, t32297, t32298, t32300)
}
