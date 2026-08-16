//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1395/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1395<F: Float>(t11981: F, t1391: F, t1392: F, t2487: F, t34631: F, t34634: F, t34636: F, t34638: F, t34640: F, t34643: F, t34645: F, t34648: F, t34650: F, t34652: F, t34659: F, t34662: F, t34665: F, t34668: F) -> F {
    let t38663 = t34631 + t34634 + F::cast_from(0.11360866949309851756e0_f64) * t2487 * t1391 * t1392 * t11981 + t34636 + t34638 - t34640 - t34643 - t34645 + t34648 + t34650 + t34652 - t34659 - t34662 + t34665 + t34668;
    t38663
}
