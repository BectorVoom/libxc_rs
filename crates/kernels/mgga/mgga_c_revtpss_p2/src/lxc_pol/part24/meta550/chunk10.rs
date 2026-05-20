//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1636/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1636<F: Float>(t39989: F, t40115: F, t40131: F, t40137: F, t87655: F, t87658: F, t87660: F, t87661: F, t87662: F, t87663: F, t87666: F, t87667: F, t87668: F, t87669: F) -> F {
    let t87952 = t87655 - t40115 + t87658 + t87660 - t40131 - t40137 + t87661 + t87662 + t87663 + t87666 + t87667 - t39989 - t87668 - t87669;
    t87952
}
