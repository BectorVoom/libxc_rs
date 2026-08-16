//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1182/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1182(t2411: f64, t6075: f64, t11064: f64, t6079: f64, t1544: f64, t890: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t11088: f64, t14618: f64, t18571: f64, t18572: f64, t18573: f64, t18574: f64, t18578: f64, t18579: f64, t18581: f64, t18582: f64, t1940: f64, t198: f64, t2403: f64, t4433: f64, t4541: f64, t4546: f64, t4556: f64, t5966: f64, t9524: f64, t9542: f64) -> f64 {
    let t18865 = t6075 * t2411;
    let t18871 = t6079 * t11064;
    let t18875 = t1544 * t890;
    let t18882 = 6.0_f64 * t11088 * t198 * t5966 - t18865 * t1940 * t890 + 2.0_f64 * t18871 * t1940 * t890 - 6.0_f64 * t18875 * t2403 * t4556 + 12.0_f64 * t4433 * t4541 * t4546 + t10592 - t10596 - t10604 - t10611 - t14618 + t18571 + t18572 - t18573 - t18574 + t18578 + t18579 + t18581 + t18582 - t9524 + t9542;
    t18882
}
