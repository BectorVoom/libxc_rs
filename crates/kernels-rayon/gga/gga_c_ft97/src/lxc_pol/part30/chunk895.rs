//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 895/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk895(t36160: f64, t36202: f64, t36239: f64, t36271: f64, t1218: f64, t1466: f64, t1479: f64, t301: f64, t34267: f64, t34276: f64, t34283: f64, t34335: f64, t36049: f64, t36057: f64, t36061: f64, t36064: f64, t36066: f64, t36069: f64, t36093: f64, t36097: f64, t36101: f64, t36105: f64, t36109: f64, t6216: f64, t6963: f64, t7024: f64, t7581: f64, t7614: f64, t7684: f64) -> (f64, f64) {
    let t36273 = t36160 + t36202 + t36239 + t36271;
    let t36275 = t1466 * t36049 / 3.0_f64 + t7581 * t7024 / 6.0_f64 + t6963 * t7614 / 6.0_f64 - t1466 * t36057 / 3.0_f64 + 4.0_f64 * t36061 - 12.0_f64 * t36064 + 8.0_f64 * t36066 + 8.0_f64 * t36069 + t36093 * t1479 / 6.0_f64 + t1466 * t36097 / 3.0_f64 - t34267 - t1218 * t7684 - 2.0_f64 * t36101 + t1466 * t36105 / 6.0_f64 - t6216 * t36109 / 9.0_f64 - t34276 + t34283 - t34335 - t301 * t36273;
    (t36273, t36275)
}
