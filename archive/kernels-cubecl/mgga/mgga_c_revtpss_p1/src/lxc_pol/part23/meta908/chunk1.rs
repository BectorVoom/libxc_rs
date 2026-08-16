//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2917/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2917<F: Float>(t23535: F, t2897: F, t918: F, t23540: F, t41401: F, t18979: F, t4606: F, t15113: F, t6120: F, t18950: F, t4598: F, t41382: F) -> (F, F, F, F, F, F) {
    let t77683 = t2897 * t23535 * t918;
    let t77686 = t41401 * t23540 * t918;
    let t77688 = t18979 * t4606;
    let t77690 = t15113 * t6120;
    let t77692 = t4598 * t18950;
    let t77695 = t41382 * t23540 * t918;
    (t77683, t77686, t77688, t77690, t77692, t77695)
}
