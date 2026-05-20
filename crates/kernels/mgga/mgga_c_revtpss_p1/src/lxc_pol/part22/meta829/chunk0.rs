//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2948/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2948<F: Float>(t1868: F, t9940: F, t5577: F, t588: F, t5585: F, t4010: F, t5591: F, t13921: F, t221: F, t4018: F, t4019: F, t2661: F, t3924: F, t3992: F, t5651: F) -> (F, F, F, F, F, F) {
    let t48347 = t9940 * t1868;
    let t48394 = F::new(16.0) * t5577 * t588;
    let t48417 = F::new(16.0) * t5585 * t588;
    let t48432 = t4010 * t5591;
    let t48445 = t4018 * t4019 * t221 * t13921;
    let t48449 = t2661 * t3992 * t5651 * t3924;
    (t48347, t48394, t48417, t48432, t48445, t48449)
}
