//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1107/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1107(t1457: f64, t632: f64, t1266: f64, t4048: f64, t424: f64, t116: f64, t14873: f64, t3116: f64, t4687: f64, t102: f64, t5390: f64, t8959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25514 = t632 * t1457;
    let t25526 = t1266 * t1457;
    let t25530 = t424 * t4048;
    let t25708 = t116 * t14873;
    let t25756 = t3116 * t4687;
    let t25813 = t8959 * t5390 * t102;
    (t25514, t25526, t25530, t25708, t25756, t25813)
}
