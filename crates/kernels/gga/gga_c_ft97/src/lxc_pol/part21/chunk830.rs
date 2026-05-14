//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 830/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk830<F: Float>(t25846: F, t369: F, t108: F, t28: F, t1286: F, t1310: F, t25570: F, t25574: F, t25577: F, t25579: F, t25584: F, t25588: F, t25591: F, t25593: F, t25596: F, t25599: F, t25602: F, t25606: F, t25612: F, t25618: F, t25622: F, t5501: F) -> (F, F, F, F) {
    let t25847 = t369 * t25846;
    let t25848 = t25847 * t108;
    let t25849 = t28 * t25848;
    let t25852 = -t5501 * t25570 / 18.0 - t5501 * t25574 / 18.0 - t25577 * t25579 / 9.0 + t25584 * t1310 / 6.0 + t25588 / 9.0 + 4.0 * t25591 + 4.0 * t25593 + 4.0 * t25596 + 4.0 * t25599 + t5501 * t25602 / 9.0 + t5501 * t25606 / 9.0 + t5501 * t25612 / 9.0 - t5501 * t25618 / 27.0 + t1286 * t25622 / 6.0 + t1286 * t25849 / 6.0;
    (t25847, t25848, t25849, t25852)
}
