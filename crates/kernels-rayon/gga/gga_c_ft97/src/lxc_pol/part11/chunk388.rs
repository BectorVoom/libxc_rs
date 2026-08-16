//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 388/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk388(t2058: f64, t2059: f64, t1730: f64, t1718: f64, t1722: f64, t1726: f64, t1733: f64, t1740: f64, t1745: f64, t1749: f64) -> (f64, f64) {
    let t2060 = t2058 * t2059;
    let t2066 = 0.11113000182098765433e-1_f64 * t1730;
    let t2071 = 0.48897200801234567903e0_f64 * t1718 - 0.88904001456790123461e-1_f64 * t1722 - 0.88904001456790123461e-1_f64 * t1726 - t2066 + 0.11113000182098765433e-1_f64 * t1733 + 0.22226000364197530865e-1_f64 * t1740 - 0.33339000546296296298e-1_f64 * t1745 + 0.16669500273148148149e-1_f64 * t1749;
    (t2060, t2071)
}
