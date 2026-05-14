//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 708/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk708<F: Float>(t12579: F, t446: F, t2075: F, t3342: F, t28: F, t89: F, t1017: F, t1986: F, t7368: F, t2223: F, t2983: F, t9049: F, t12346: F, t12353: F, t12357: F, t12359: F, t12362: F, t12366: F, t12564: F, t12568: F, t12571: F, t12574: F, t12577: F, t8799: F, t8802: F, t9059: F, t9062: F, t9072: F) -> (F, F, F, F, F, F) {
    let t12580 = t446 * t12579;
    let t12582 = t3342 * t2075;
    let t12584 = t89 * t28 * t12582;
    let t12587 = t7368 * t1017 * t1986;
    let t12589 = t89 * t28 * t12587;
    let t12590 = t2983 * t2223;
    let t12591 = t9049 * t12590;
    let t12592 = t446 * t12591;
    let t12595 = -t12346 + t8799 / 54.0 + t8802 / 81.0 - t9059 / 27.0 + 2.0 / 3.0 * t12353 - t12357 + 11.0 / 27.0 * t12359 - 2.0 / 81.0 * t12362 - t9072 + t12366 - t12564 / 6.0 - t12568 / 9.0 - 2.0 / 27.0 * t12571 + t12574 / 9.0 + 4.0 / 9.0 * t12577 - t12580 / 27.0 + t12584 / 3.0 - t12589 + 2.0 / 27.0 * t12592 - t9062 / 27.0;
    (t12580, t12584, t12589, t12590, t12592, t12595)
}
