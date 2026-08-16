//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 762/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk762(t1853: f64, t3615: f64, t1022: f64, t2925: f64, t7290: f64, t35450: f64, t11576: f64, t296: f64, t2101: f64, t3614: f64, t835: f64, t1023: f64, t35385: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35583 = t3615 * t1853;
    let t35610 = t1022 * t2925;
    let t35611 = t7290 * t35610;
    let t35623 = t7290 * t35450;
    let t35659 = t296 * t11576;
    let t35682 = t2101 * t3614;
    let t35709 = t835 * t11576;
    let t35719 = t1023 * t35385;
    (t35583, t35611, t35623, t35659, t35682, t35709, t35719)
}
