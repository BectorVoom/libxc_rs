//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 800/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk800<F: Float>(t12587: F, t28: F, t89: F, t2223: F, t2983: F, t9049: F, t446: F, t12346: F, t12353: F, t12357: F, t12359: F, t12362: F, t12366: F, t12564: F, t12568: F, t12571: F, t12574: F, t12577: F, t12580: F, t12584: F, t8799: F, t8802: F, t9059: F, t9062: F, t9072: F) -> (F, F, F, F) {
    let t12589 = t89 * t28 * t12587;
    let t12590 = t2983 * t2223;
    let t12591 = t9049 * t12590;
    let t12592 = t446 * t12591;
    let t12595 = -t12346 + t8799 / F::new(54.0) + t8802 / F::new(81.0) - t9059 / F::new(27.0) + F::new(2.0) / F::new(3.0) * t12353 - t12357 + F::new(11.0) / F::new(27.0) * t12359 - F::new(2.0) / F::new(81.0) * t12362 - t9072 + t12366 - t12564 / F::new(6.0) - t12568 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t12571 + t12574 / F::new(9.0) + F::new(4.0) / F::new(9.0) * t12577 - t12580 / F::new(27.0) + t12584 / F::new(3.0) - t12589 + F::new(2.0) / F::new(27.0) * t12592 - t9062 / F::new(27.0);
    (t12589, t12590, t12592, t12595)
}
