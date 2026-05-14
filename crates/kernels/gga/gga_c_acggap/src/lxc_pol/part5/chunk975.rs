//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 975/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk975<F: Float>(t11653: F, t11659: F, t11602: F, t11652: F, t11657: F, t1268: F, t1427: F, t14947: F, t1674: F, t1679: F, t1680: F, t1734: F, t19409: F, t19418: F, t2831: F, t467: F, t5403: F, t6589: F, t6596: F, t694: F) -> (F, F, F) {
    let t19422 = 0.43374325201206959367e-1 * t11653;
    let t19423 = 0.10843581300301739842e-1 * t11659;
    let t19424 = 2.0 * t1268 * t1679 * t6596 + 24.0 * t1427 * t1674 * t5403 - 2.0 * t1679 * t19418 * t467 - 12.0 * t1680 * t19409 * t694 + 3.0 * t1734 * t2831 * t694 + 24.0 * t14947 * t6589 - t11602 - t11652 + t11657 - t19422 + t19423;
    (t19422, t19423, t19424)
}
