//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1400/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1400<F: Float>(t1531: F, t34567: F, t34706: F, t34709: F, t34712: F, t34714: F, t34717: F, t34720: F, t34726: F, t34730: F, t34733: F, t34737: F, t34740: F, t34743: F, t34746: F, t34749: F, t34752: F, t7025: F) -> F {
    let t34756 = -t34706 - t34709 - t34712 + t34714 + t34717 + t34720 - t34726 + t34730 + t34733 - t34737 - t34740 + t34743 + t34746 - t34749 - t34752 + F::cast_from(0.21450293971110256002e1_f64) * t7025 * t1531 * t34567;
    t34756
}
