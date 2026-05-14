//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 734/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk734<F: Float>(t2409: F, t904: F, t2923: F, t231: F, t2918: F, t9571: F, t4342: F, t9592: F, t9556: F, t10305: F, t10308: F, t10316: F, t2417: F, t9558: F, t9560: F, t9562: F, t9564: F, t9574: F, t9580: F, t9585: F, t9589: F, t9594: F, t9598: F) -> (F, F, F, F) {
    let t10870 = t2409 * t904;
    let t10871 = t2923 * t10870;
    let t10875 = t231 * t2918 * t9571;
    let t10877 = t4342 * t9592;
    let t10883 = 0.44934037037037037036e0 * t9556;
    let t10894 = 0.1760655e0 * t10305 - 0.352131e0 * t10308 * t2417 + 0.234754e0 * t10316 - t10883 - 0.19257444444444444444e0 * t9558 + 0.9628722222222222222e-1 * t9560 - 0.28886166666666666666e0 * t9562 + 0.14443083333333333333e0 * t9564 - 0.1604787037037037037e0 * t9574 + 0.57772333333333333332e0 * t9580 - 0.28886166666666666666e0 * t9585 - 0.86658499999999999998e0 * t9589 + 0.86658499999999999998e0 * t9594 - 0.14443083333333333333e0 * t9598;
    (t10871, t10875, t10877, t10894)
}
