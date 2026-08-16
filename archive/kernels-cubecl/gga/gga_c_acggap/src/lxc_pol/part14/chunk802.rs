//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 802/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk802<F: Float>(t8500: F, t8552: F, t8594: F, t8627: F, t8674: F, t8702: F, t8728: F, t8750: F, t8765: F, t8797: F, t8834: F, t8872: F, t8900: F, t8936: F, t8959: F, t8989: F) -> F {
    let t8993 = t8500 + t8552 + t8594 + t8627 + t8674 + t8702 + t8728 + t8750 + t8765 + t8797 + t8834 + t8872 + t8900 + t8936 + t8959 + t8989;
    t8993
}
