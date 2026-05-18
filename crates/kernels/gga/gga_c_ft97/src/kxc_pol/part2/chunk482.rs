//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 482/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk482<F: Float>(t2789: F, t845: F, t91: F, t2652: F, t2655: F, t2658: F, t2663: F, t2668: F, t2673: F, t2677: F, t2685: F, t2742: F, t2758: F) -> (F, F, F) {
    let t2791 = t91 * t845 * t2789;
    let t2793 = F::new(4.0) / F::new(9.0) * t2652;
    let t2801 = -t2758 / F::new(4.0) + t2791 / F::new(2.0) + t2793 + F::new(2.0) / F::new(9.0) * t2655 + F::new(2.0) / F::new(3.0) * t2658 - F::new(2.0) / F::new(9.0) * t2663 + F::new(2.0) / F::new(3.0) * t2668 + F::new(2.0) / F::new(3.0) * t2673 - t2677 / F::new(3.0) + F::new(2.0) * t2685 - t2742;
    (t2791, t2793, t2801)
}
