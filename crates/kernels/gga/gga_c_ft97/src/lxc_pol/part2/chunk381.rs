//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 381/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk381<F: Float>(t1701: F, t2044: F, t137: F, t548: F, t135: F, t554: F, t1730: F, t1718: F, t1722: F, t1726: F, t1733: F, t1740: F, t1745: F, t1749: F) -> (F, F, F, F, F, F, F) {
    let t2045 = t1701 * t2044;
    let t2057 = F::new(1.0) / t548 / t137;
    let t2058 = t135 * t2057;
    let t2059 = t554 * t554;
    let t2060 = t2058 * t2059;
    let t2066 = F::cast_from(0.11113000182098765433e-1_f64) * t1730;
    let t2071 = F::cast_from(0.48897200801234567903e0_f64) * t1718 - F::cast_from(0.88904001456790123461e-1_f64) * t1722 - F::cast_from(0.88904001456790123461e-1_f64) * t1726 - t2066 + F::cast_from(0.11113000182098765433e-1_f64) * t1733 + F::cast_from(0.22226000364197530865e-1_f64) * t1740 - F::cast_from(0.33339000546296296298e-1_f64) * t1745 + F::cast_from(0.16669500273148148149e-1_f64) * t1749;
    (t2045, t2057, t2058, t2059, t2060, t2066, t2071)
}
