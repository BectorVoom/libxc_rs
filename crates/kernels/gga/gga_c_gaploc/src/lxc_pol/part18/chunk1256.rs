//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1256/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1256<F: Float>(t1980: F, t8792: F, t10024: F, t10843: F, t2013: F, t11038: F, t4614: F, t813: F, t10964: F, t2194: F, t10717: F, t833: F) -> (F, F, F, F, F, F) {
    let t32757 = t1980 * t8792;
    let t32758 = t32757 * t10024;
    let t32759 = F::new(0.89376224879626066674e-1) * t32758;
    let t32760 = t2013 * t10843;
    let t32761 = F::new(0.51123901271894332902e0) * t32760;
    let t32764 = F::new(0.12269736305254639897e2) * t813 * t4614 * t11038;
    let t32766 = F::new(0.12269736305254639897e2) * t2194 * t10964;
    let t32769 = F::new(0.30674340763136599742e2) * t833 * t4614 * t10717;
    (t32757, t32759, t32761, t32764, t32766, t32769)
}
