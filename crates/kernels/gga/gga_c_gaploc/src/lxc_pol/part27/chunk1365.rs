//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1365/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1365<F: Float>(t105: F, t1083: F, t11978: F, t11988: F, t12019: F, t1324: F, t169: F, t172: F, t29892: F, t31570: F, t31575: F, t31577: F, t31581: F, t31584: F, t31589: F, t31594: F, t31600: F, t3692: F, t3701: F, t380: F, t3818: F, t3822: F, t38313: F, t452: F, t6313: F) -> F {
    let t38337 = F::new(0.7588001769513639893e-1) * t380 * t12019 + F::new(0.7588001769513639893e-1) * t1083 * t3692 + t29892 + t31570 + t31575 + F::new(0.7588001769513639893e-1) * t3818 * t11988 + F::new(0.56910013271352299198e-1) * t3822 * t3701 * t1324 - F::new(0.2276400530854091968e0) * t6313 * t11978 + t31577 + t31581 + t31584 - t31589 - t31594 - t31600 + F::new(0.28455006635676149599e-1) * t105 * t452 * t38313 * t169 * t172;
    t38337
}
