//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3469/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3469<F: Float>(t63586: F, t63589: F, t63592: F, t63596: F, t63600: F, t63607: F, t63609: F, t63612: F, t63615: F, t63618: F, t63620: F, t63622: F, t63625: F) -> F {
    let t65389 = t63586 + t63589 + t63592 + t63596 + t63600 - t63607 + t63609 + t63612 + t63615 - t63618 + t63620 + t63622 + t63625;
    t65389
}
