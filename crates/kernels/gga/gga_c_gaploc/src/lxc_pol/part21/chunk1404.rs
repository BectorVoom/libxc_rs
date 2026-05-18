//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1404/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1404<F: Float>(t31291: F, t34983: F, t34986: F, t34991: F, t34994: F, t34996: F, t34999: F, t35021: F, t35024: F, t35027: F, t35034: F, t35038: F, t35041: F, t35044: F, t35048: F, t35052: F) -> F {
    let t38812 = t34983 - t34986 + t34991 + t34994 - t34996 - t34999 - t31291 + t35021 + t35024 + t35027 - t35034 - t35038 + t35041 - t35044 - t35048 - t35052;
    t38812
}
