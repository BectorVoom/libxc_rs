//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 365/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk365<F: Float>(t11: F, t1689: F, t2034: F, t1691: F, t1696: F, t139: F, t1354: F, t542: F, t1702: F, t554: F, t1701: F, t137: F, t548: F, t135: F, t1730: F, t1718: F, t1722: F, t1726: F, t1733: F, t1740: F, t1745: F, t1749: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2035 = t1689 * t11;
    let t2036 = t2034 * t2035;
    let t2037 = t1691 * t1696;
    let t2038 = t2037 * t139;
    let t2043 = t542 * t1354;
    let t2044 = t1702 * t554;
    let t2045 = t1701 * t2044;
    let t2057 = 1.0 / t548 / t137;
    let t2058 = t135 * t2057;
    let t2059 = t554 * t554;
    let t2060 = t2058 * t2059;
    let t2066 = 0.11113000182098765433e-1 * t1730;
    let t2071 = 0.48897200801234567903e0 * t1718 - 0.88904001456790123461e-1 * t1722 - 0.88904001456790123461e-1 * t1726 - t2066 + 0.11113000182098765433e-1 * t1733 + 0.22226000364197530865e-1 * t1740 - 0.33339000546296296298e-1 * t1745 + 0.16669500273148148149e-1 * t1749;
    (t2035, t2036, t2037, t2038, t2043, t2044, t2045, t2057, t2058, t2059, t2060, t2066, t2071)
}
