//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1030/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1030<F: Float>(t121167: F, t25304: F, t25946: F, t32715: F, t10073: F, t25938: F, t32689: F, t122281: F, t1955: F, t121202: F, t122317: F, t32710: F, t121184: F, t8477: F, t32673: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t122413 = 0.26773803678175077507e-4 * t121167;
    let t122435 = 0.45699670022203476294e-2 * t25304 * t32715 * t25946;
    let t122438 = 0.4818682326780666368e-3 * t10073 * t32689 * t25938;
    let t122443 = t1955 * t122281;
    let t122451 = 0.14932895752263002547e-1 * t121202;
    let t122454 = 0.33852964522850660984e-1 * t32710 * t122317;
    let t122455 = t8477 * t121184;
    let t122463 = t32673 * t72 * t686;
    (t122413, t122435, t122438, t122443, t122451, t122454, t122455, t122463)
}
