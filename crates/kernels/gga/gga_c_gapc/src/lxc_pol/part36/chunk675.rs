//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 675/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk675<F: Float>(t1504: F, t2880: F, t8589: F, t1461: F, t4043: F, t1030: F, t3141: F, t5059: F, t1044: F, t1971: F, t1743: F, t5722: F, t458: F, t4925: F, t3104: F, t568: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8590 = t2880 * t1504;
    let t8591 = t8589 * t8590;
    let t8619 = t1461 * t4043;
    let t8620 = t1030 * t8619;
    let t8621 = t3141 * t5059;
    let t8622 = t8620 * t8621;
    let t8624 = t1971 * t1044;
    let t8625 = t1743 * t8624;
    let t8626 = t8625 * t5722;
    let t8628 = t4925 * t458;
    let t8629 = t3104 * t8628;
    let t8631 = t4925 * t568;
    (t8591, t8619, t8620, t8621, t8622, t8624, t8626, t8629, t8631)
}
