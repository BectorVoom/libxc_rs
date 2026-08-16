//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1140/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1140(t11902: f64, t15938: f64, t11990: f64, t19139: f64, t2597: f64, t1: f64, t33543: f64, t1084: f64, t33961: f64, t11311: f64, t11791: f64, t2520: f64) -> (f64, f64, f64, f64, f64) {
    let t34100 = t11902 * t15938;
    let t34104 = t11990 * t2597 * t19139;
    let t34106 = t33543 * t1;
    let t34108 = t1084 * t34106 * t33961;
    let t34111 = t2520 * t11311 * t11791;
    (t34100, t34104, t34106, t34108, t34111)
}
