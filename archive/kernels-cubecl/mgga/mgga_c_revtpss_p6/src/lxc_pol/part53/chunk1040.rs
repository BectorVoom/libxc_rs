//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1040/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1040<F: Float>(t1937: F, t27060: F, t118: F, t1310: F, t1453: F, t1932: F, t2007: F, t2127: F, t2163: F, t32791: F, t32815: F, t32823: F, t32824: F, t32837: F, t508: F, t569: F, t649: F, t6983: F, t7221: F, t7584: F, t7683: F, t8741: F, t8756: F, t8761: F) -> F {
    let t32840 = t27060 * t1937;
    let t32842 = -t118 * t32791 - t1310 * t8741 + t1453 * t8761 - t1932 * t7683 - t2007 * t7584 - t2127 * t7221 - t2163 * t6983 - t32815 * t508 + t32837 * t569 - t649 * t8756 + t32823 + t32824 - F::cast_from(2.0_f64) * t32840;
    t32842
}
