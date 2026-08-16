//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1030/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1030(t115919: f64, t115920: f64, t115922: f64, t115924: f64, t115927: f64, t115929: f64, t115942: f64, t115946: f64, t115948: f64, t115959: f64, t115965: f64, t115968: f64, t117531: f64, t1266: f64, t2114: f64, t23918: f64, t23938: f64, t24428: f64, t24932: f64, t32349: f64, t510: f64, t7061: f64, t7266: f64, t7271: f64) -> f64 {
    let t117659 = -t117531 * t510 - 2.0_f64 * t1266 * t32349 - t2114 * t24428 - 2.0_f64 * t23918 * t7266 - 4.0_f64 * t23938 * t7271 - 4.0_f64 * t24932 * t7061 - t115919 - t115920 + t115922 + t115924 - t115927 - t115929 - t115942 - t115946 - t115948 + t115959 + t115965 - t115968;
    t117659
}
