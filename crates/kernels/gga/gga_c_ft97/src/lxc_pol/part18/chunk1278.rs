//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1278/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1278<F: Float>(t23405: F, t26805: F, t1349: F, t26770: F, t376: F, t26574: F, t5766: F, t1637: F, t6621: F, t1017: F, t1023: F, t12945: F, t13234: F, t1389: F, t2228: F, t23408: F, t23413: F, t23925: F, t24057: F, t26538: F, t26540: F, t26546: F, t26815: F, t26823: F, t28: F, t3450: F, t5772: F, t5773: F, t5778: F, t9432: F) -> (F,) {
    let t104426 = t23405 * t26805 / 27.0;
    let t104432 = t1349 * t376 * t26770 / 9.0;
    let t104434 = t5766 * t26574 / 9.0;
    let t104436 = t1349 * t1637 * t6621;
    let t104442 = -2.0 / 3.0 * t5766 * t26540 - 2.0 / 3.0 * t1349 * t28 * t23925 * t26538 - t1349 * t28 * t5778 * t2228 * t1017 / 3.0 + 2.0 * t5772 * t9432 * t23408 * t3450 + 2.0 * t23413 * t26815 - t23413 * t26823 / 9.0 + t104426 - 2.0 / 3.0 * t5766 * t26546 - t1023 * t24057 - t104432 - t104434 + 2.0 / 27.0 * t104436 - t13234 * t1389 + t5772 * t9432 * t5773 * t12945;
    (t104442,)
}
