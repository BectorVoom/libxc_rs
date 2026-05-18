//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1051/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1051<F: Float>(t11547: F, t11552: F, t11564: F, t11567: F, t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11599: F, t11602: F, t11605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12129 = F::new(0.1422820120100248667e-7) * t11547;
    let t12131 = F::new(0.11594181388521408695e-4) * t11552;
    let t12135 = F::new(0.11594181388521408695e-4) * t11564;
    let t12136 = F::new(0.21720231316129303386e-4) * t11567;
    let t12137 = F::new(0.2318836277704281739e-4) * t11570;
    let t12138 = F::new(0.21720231316129303386e-4) * t11572;
    let t12139 = F::new(0.34752370105806885418e-3) * t11574;
    let t12140 = F::new(0.28960308421505737848e-5) * t11581;
    let t12141 = F::new(0.1349435763888888889e-4) * t11584;
    let t12144 = F::new(0.67530371184977617164e-6) * t11599;
    let t12145 = F::new(0.13506074236995523433e-5) * t11602;
    let t12146 = F::new(0.21103240995305505364e-7) * t11605;
    (t12129, t12131, t12135, t12136, t12137, t12138, t12139, t12140, t12141, t12144, t12145, t12146)
}
