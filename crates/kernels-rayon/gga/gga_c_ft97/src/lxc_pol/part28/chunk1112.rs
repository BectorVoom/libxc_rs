//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1112/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1112(t2035: f64, t3379: f64, t7318: f64, t136825: f64, t32774: f64, t34910: f64, t32767: f64, t34906: f64, t1013: f64, t32186: f64, t52: f64, t3404: f64, t7182: f64) -> (f64, f64, f64, f64, f64) {
    let t147497 = t2035 * t7318 * t3379;
    let t147505 = t32774 * t136825 * t34910;
    let t147511 = t32767 * t136825 * t34906;
    let t147517 = t52 * t32186 * t1013;
    let t147521 = t52 * t7182 * t3404;
    (t147497, t147505, t147511, t147517, t147521)
}
