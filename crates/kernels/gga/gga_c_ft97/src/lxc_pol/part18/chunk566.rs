//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 566/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk566<F: Float>(t1023: F, t1349: F, t1362: F, t1389: F, t149: F, t5771: F, t5772: F, t6580: F, t6584: F, t6589: F, t6618: F, t6622: F, t6691: F, t6704: F, t6709: F, t6719: F, t6723: F, t6725: F) -> (F,) {
    let t6731 = t6580 * t1362 / 6.0 - t5771 - t5772 * t6584 / 18.0 - t1349 * t6589 / 3.0 + t1349 * t6618 / 6.0 + t1349 * t6622 / 6.0 - t1023 * t1389 - t149 * t6723 + 2.0 * t6725 - 2.0 * t6691 - 2.0 * t6704 + 4.0 * t6709 - 2.0 * t6719;
    (t6731,)
}
