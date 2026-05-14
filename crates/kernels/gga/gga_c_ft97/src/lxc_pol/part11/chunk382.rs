//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 382/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk382<F: Float>(t2058: F, t2059: F, t1730: F, t1718: F, t1722: F, t1726: F, t1733: F, t1740: F, t1745: F, t1749: F) -> (F, F) {
    let t2060 = t2058 * t2059;
    let t2066 = 0.11113000182098765433e-1 * t1730;
    let t2071 = 0.48897200801234567903e0 * t1718 - 0.88904001456790123461e-1 * t1722 - 0.88904001456790123461e-1 * t1726 - t2066 + 0.11113000182098765433e-1 * t1733 + 0.22226000364197530865e-1 * t1740 - 0.33339000546296296298e-1 * t1745 + 0.16669500273148148149e-1 * t1749;
    (t2060, t2071)
}
