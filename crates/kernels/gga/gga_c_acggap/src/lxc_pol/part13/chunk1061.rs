//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1061/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1061<F: Float>(t694: F, t8379: F, t560: F, t922: F, t105: F, t1953: F, t2163: F, t24605: F, t2541: F, t29938: F, t29950: F, t33393: F, t33397: F, t33403: F, t33409: F, t33412: F, t33441: F, t33478: F, t33517: F, t33558: F, t33588: F, t33628: F, t33666: F, t33711: F, t33755: F, t33775: F, t33810: F, t36425: F, t36463: F, t36489: F, t36528: F, t36566: F, t469: F, t5399: F, t567: F, t7301: F, t8372: F, t8387: F, t9096: F, t9097: F) -> (F,) {
    let t36575 = 6.0 * t694 * t8379;
    let t36577 = t560 * t922;
    let t36585 = -2.0 * t567 * t2163 * t5399 + 3.0 * t567 * t1953 * t33393 - 12.0 * t8372 * t2541 * t33397 - t33403 + 4.0 * t9096 * t9097 * t24605 + t33409 + t33412 + t567 * t105 * (t33441 + t33478 + t33517 + t33558 + t33588 + t33628 + t33666 + t33711 + t33755 + t33775 + t33810 + t36425 + t36463 + t36489 + t36528 + t36566) * t469 - t36575 - 6.0 * t29938 - 6.0 * t8372 * t2541 * t36577 + 6.0 * t29950 + 3.0 * t567 * t8387 * t7301;
    (t36585,)
}
