//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1170/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1170<F: Float>(t104462: F, t5956: F, t11593: F, t140325: F, t144: F, t148678: F, t148960: F, t148964: F, t148966: F, t167: F, t1901: F, t2185: F, t2210: F, t23478: F, t3052: F, t33055: F, t34918: F, t446: F, t574: F, t5975: F, t605: F, t609: F, t6630: F, t6695: F, t6699: F, t95767: F) -> (F, F) {
    let t148977 = t104462 * t5956;
    let t148997 = -t446 * t144 * t148960 / F::new(3.0) + t148964 / F::new(9.0) + t148966 / F::new(9.0) + F::new(4.0) / F::new(3.0) * t446 * t2185 * t5975 * t6630 - F::new(4.0) / F::new(9.0) * t140325 + F::new(2.0) / F::new(3.0) * t446 * t574 * t23478 * t6699 + F::new(4.0) / F::new(3.0) * t446 * t144 * t148977 + F::new(2.0) / F::new(3.0) * t446 * t2185 * t167 * t148678 + t446 * t574 * t605 * t34918 * t609 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t11593 * t2210 * t33055 * t3052 + F::new(2.0) / F::new(9.0) * t1901 * t95767 * t6695;
    (t148977, t148997)
}
