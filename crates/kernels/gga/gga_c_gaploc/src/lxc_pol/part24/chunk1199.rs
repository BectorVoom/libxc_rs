//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1199/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1199<F: Float>(t31590: F, t426: F, t2268: F, t535: F, t10227: F, t23927: F, t10276: F, t4141: F, t10224: F, t1083: F, t30126: F, t30129: F, t30132: F, t30135: F, t30145: F, t30148: F, t30152: F, t30169: F, t30171: F, t30173: F, t3341: F, t380: F) -> F {
    let t32005 = t31590 * t426;
    let t32008 = F::new(0.56910013271352299198e-1) * t2268 * t535 * t32005;
    let t32009 = t23927 * t10227;
    let t32010 = F::new(0.23712505529730124666e-2) * t32009;
    let t32012 = F::new(0.9485002211892049866e-2) * t4141 * t10276;
    let t32017 = t30126 + t30129 - t30132 + t32008 + t32010 + t30135 - t30145 + t30148 - t30152 - t32012 - t30169 + F::new(0.7588001769513639893e-1) * t1083 * t3341 + F::new(0.7588001769513639893e-1) * t380 * t10224 + t30171 - t30173;
    t32017
}
