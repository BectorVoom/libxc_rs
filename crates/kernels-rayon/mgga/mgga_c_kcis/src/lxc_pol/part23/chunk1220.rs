//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1220/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1220(t17323: f64, t27494: f64, t27503: f64, t48058: f64, t12940: f64, t18268: f64, t2268: f64, t27697: f64, t27705: f64, t28655: f64, t28698: f64, t40556: f64, t40662: f64, t4475: f64, t4481: f64, t51097: f64, t52930: f64, t52955: f64, t6225: f64, t8001: f64, t8240: f64, t8251: f64, t94824: f64, t97654: f64, t97657: f64, t97852: f64, t97854: f64, t97856: f64, t97862: f64, t97870: f64) -> (f64, f64, f64) {
    let t97875 = 4.0_f64 * t27494 * t17323;
    let t97877 = 6.0_f64 * t48058 * t27503;
    let t97880 = -6.0_f64 * t12940 * t4481 * t8251 + 2.0_f64 * t18268 * t27697 - t2268 * t52955 - 6.0_f64 * t27705 * t51097 - 12.0_f64 * t28655 * t40662 - 2.0_f64 * t28698 * t4475 + 2.0_f64 * t40556 * t8240 + 4.0_f64 * t52930 * t8001 + 4.0_f64 * t6225 * t94824 - t97654 - t97657 + t97852 + t97854 - t97856 - t97862 - t97870 - t97875 + t97877;
    (t97875, t97877, t97880)
}
