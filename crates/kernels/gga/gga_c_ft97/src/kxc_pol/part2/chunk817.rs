//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 817/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk817<F: Float>(t11013: F, t3499: F, t12298: F, t2102: F, t1775: F, t3503: F, t3507: F, t11755: F, t11761: F, t12775: F, t12778: F, t12781: F, t12784: F, t12788: F, t12793: F, t12797: F, t12800: F, t12803: F, t12807: F, t12809: F, t12812: F, t12816: F, t12817: F, t12820: F, t12824: F, t3051: F, t3139: F, t462: F, t92: F) -> F {
    let t12827 = t3499 * t11013;
    let t12830 = t2102 * t12298;
    let t12834 = F::new(2.0) / F::new(9.0) * t1775 * t3503;
    let t12836 = F::new(4.0) / F::new(9.0) * t1775 * t3507;
    let t12837 = -F::new(2.0) / F::new(3.0) * t462 * t12775 - F::new(2.0) / F::new(3.0) * t462 * t12778 - F::new(2.0) * t462 * t12781 + F::new(4.0) / F::new(3.0) * t462 * t12784 - F::new(4.0) / F::new(3.0) * t11761 * t12788 - F::new(4.0) / F::new(3.0) * t11761 * t12793 + F::new(4.0) / F::new(9.0) * t11755 * t12797 + F::new(2.0) / F::new(3.0) * t462 * t12800 + F::new(8.0) / F::new(3.0) * t3139 * t12803 - t92 * t12807 - F::new(4.0) / F::new(9.0) * t12809 - F::new(2.0) / F::new(3.0) * t3051 * t12812 + t12816 + F::new(4.0) / F::new(3.0) * t3139 * t12817 - F::new(2.0) / F::new(9.0) * t462 * t12820 - F::new(10.0) / F::new(27.0) * t462 * t12824 - F::new(8.0) / F::new(9.0) * t3139 * t12827 + t462 * t12830 / F::new(3.0) - t12834 - t12836;
    t12837
}
