//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1090/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1090<F: Float>(t1742: F, t435: F, t1111: F, t384: F, t398: F, t1008: F, t6211: F, t1416: F, t301: F, t1137: F, t5598: F, t5632: F, t3706: F, t506: F, t1797: F, t3573: F) -> (F, F, F, F, F, F, F, F) {
    let t22099 = t1742 * t435;
    let t22102 = t384 * t398 * t22099 * t1111;
    let t22105 = t1008 * t6211;
    let t22107 = t1416 * t301;
    let t22112 = t1137 * t5598;
    let t22114 = t1137 * t5632;
    let t22120 = t3706 * t506;
    let t22125 = t3573 * t1797;
    (t22099, t22102, t22105, t22107, t22112, t22114, t22120, t22125)
}
