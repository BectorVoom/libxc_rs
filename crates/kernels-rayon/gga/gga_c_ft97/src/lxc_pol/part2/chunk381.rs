//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 381/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk381(t1701: f64, t2044: f64, t137: f64, t548: f64, t135: f64, t554: f64, t1730: f64, t1718: f64, t1722: f64, t1726: f64, t1733: f64, t1740: f64, t1745: f64, t1749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2045 = t1701 * t2044;
    let t2057 = 1.0_f64 / t548 / t137;
    let t2058 = t135 * t2057;
    let t2059 = t554 * t554;
    let t2060 = t2058 * t2059;
    let t2066 = 0.11113000182098765433e-1_f64 * t1730;
    let t2071 = 0.48897200801234567903e0_f64 * t1718 - 0.88904001456790123461e-1_f64 * t1722 - 0.88904001456790123461e-1_f64 * t1726 - t2066 + 0.11113000182098765433e-1_f64 * t1733 + 0.22226000364197530865e-1_f64 * t1740 - 0.33339000546296296298e-1_f64 * t1745 + 0.16669500273148148149e-1_f64 * t1749;
    (t2045, t2057, t2058, t2059, t2060, t2066, t2071)
}
