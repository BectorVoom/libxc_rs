//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 879/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk879<F: Float>(t12329: F, t687: F, t10526: F, t1112: F, t10529: F, t3483: F, t3480: F, t3537: F, t3859: F, t4908: F, t4915: F, t1616: F, t1611: F, t3873: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12330 = t12329 * t687;
    let t12331 = t10526 * t1112;
    let t12332 = 2.0 * t12331;
    let t12333 = t10529 * t3483;
    let t12334 = 4.0 * t12333;
    let t12335 = t3480 * t3537;
    let t12336 = 2.0 * t12335;
    let t12337 = t4908 * t3859;
    let t12338 = 2.0 * t12337;
    let t12339 = t3859 * t687;
    let t12340 = t4915 * t12339;
    let t12341 = 6.0 * t12340;
    let t12342 = t1112 * t3537;
    let t12343 = t1616 * t12342;
    let t12344 = 4.0 * t12343;
    let t12345 = t1611 * t3873;
    (t12330, t12331, t12332, t12333, t12334, t12335, t12336, t12337, t12338, t12339, t12340, t12341, t12342, t12343, t12344, t12345)
}
