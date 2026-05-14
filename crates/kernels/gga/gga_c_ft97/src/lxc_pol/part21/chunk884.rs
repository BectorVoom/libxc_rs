//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 884/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk884<F: Float>(t26791: F, t5779: F, t28: F, t379: F, t6587: F, t24080: F, t1969: F, t24102: F, t925: F, t1349: F, t24088: F, t24095: F, t26597: F, t26599: F, t26771: F, t26777: F, t26780: F, t26785: F, t26789: F, t564: F, t5766: F, t5772: F, t5849: F, t6580: F, t6622: F, t6723: F) -> (F, F, F, F, F, F) {
    let t26792 = t26791 * t5779;
    let t26793 = t28 * t26792;
    let t26800 = t6587 * t379;
    let t26801 = t24080 * t26800;
    let t26805 = t1969 * t24102 * t925;
    let t26808 = -2.0 * t26597 - 2.0 * t26599 + t1349 * t26771 / 6.0 + t6580 * t5849 / 6.0 - t564 * t6723 - 2.0 * t26777 + t1349 * t26780 / 6.0 - t5772 * t26785 / 18.0 + t26789 / 9.0 - t1349 * t26793 / 3.0 + t5766 * t6622 / 6.0 - t24088 / 18.0 - t24095 / 18.0 + t5772 * t26801 / 9.0 - t5772 * t26805 / 18.0;
    (t26792, t26793, t26800, t26801, t26805, t26808)
}
