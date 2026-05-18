//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 484/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk484<F: Float>(t1882: F, t877: F, t2652: F, t2655: F, t2658: F, t2663: F, t2668: F, t2673: F, t2677: F, t2685: F, t2742: F, t2758: F, t2791: F) -> (F, F, F) {
    let t2819 = t1882 * t877;
    let t2823 = F::new(4.0) / F::new(27.0) * t2652;
    let t2832 = -t2758 / F::new(12.0) + t2791 / F::new(6.0) + t2823 + F::new(2.0) / F::new(27.0) * t2655 + F::new(2.0) / F::new(9.0) * t2658 - F::new(2.0) / F::new(27.0) * t2663 + F::new(2.0) / F::new(9.0) * t2668 + F::new(2.0) / F::new(9.0) * t2673 - t2677 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t2685 - t2742 / F::new(3.0);
    (t2819, t2823, t2832)
}
