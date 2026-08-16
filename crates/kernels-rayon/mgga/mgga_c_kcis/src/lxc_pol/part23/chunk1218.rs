//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1218/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1218(t12940: f64, t1629: f64, t1636: f64, t17710: f64, t18268: f64, t18352: f64, t2128: f64, t27673: f64, t27693: f64, t28666: f64, t28698: f64, t40653: f64, t4480: f64, t4481: f64, t6256: f64, t7998: f64, t8010: f64, t8240: f64, t97635: f64, t97637: f64, t97638: f64, t97641: f64, t97643: f64, t97645: f64, t97647: f64, t97650: f64, t97652: f64, t97700: f64, t97740: f64, t97781: f64, t97824: f64) -> f64 {
    let t97845 = -t97635 - 12.0_f64 * t12940 * t28666 * t1636 - t97637 + t97638 - t1629 * (t97700 + t97740 + t97781 + t97824) + 2.0_f64 * t4480 * t27693 * t2128 - t97641 + 4.0_f64 * t4480 * t28698 * t1636 - t7998 * t18352 + 4.0_f64 * t18268 * t27673 - t97643 - t97645 + t97647 + 4.0_f64 * t4480 * t8010 * t6256 + 24.0_f64 * t40653 * t8240 * t4481 + t97650 + t97652 - 2.0_f64 * t17710 * t8010;
    t97845
}
