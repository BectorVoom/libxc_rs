//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 566/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk566<F: Float>(t4012: F, t5627: F, t828: F, t3826: F, t187: F, t5566: F, t1856: F, t72: F, t757: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t5546: F, t5548: F, t5568: F, t5570: F, t5573: F) -> (F, F, F, F, F) {
    let t5629 = t4012 * t828 * t5627;
    let t5632 = 0.18311447306006545054e-3 * t3826;
    let t5634 = 0.19751673498613801407e-1 * t5566 * t187;
    let t5635 = t1856 * t72;
    let t5636 = t5635 * t757;
    let t5637 = 0.18311447306006545054e-3 * t5636;
    let t5638 = -t2569 + t2579 + t2587 - t2522 + t5546 - t5548 + t5568 + t5570 - t5573 - t5632 - t2562 + t5634 - t5637;
    (t5629, t5632, t5634, t5637, t5638)
}
