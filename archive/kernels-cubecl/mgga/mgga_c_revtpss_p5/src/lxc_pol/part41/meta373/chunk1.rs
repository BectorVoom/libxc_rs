//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1226/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1226<F: Float>(t2411: F, t6075: F, t11064: F, t6079: F, t1544: F, t890: F, t10592: F, t10596: F, t10604: F, t10611: F, t11088: F, t14618: F, t18571: F, t18572: F, t18573: F, t18574: F, t18578: F, t18579: F, t18581: F, t18582: F, t1940: F, t198: F, t2403: F, t4433: F, t4541: F, t4546: F, t4556: F, t5966: F, t9524: F, t9542: F) -> F {
    let t18865 = t6075 * t2411;
    let t18871 = t6079 * t11064;
    let t18875 = t1544 * t890;
    let t18882 = F::cast_from(6.0_f64) * t11088 * t198 * t5966 - t18865 * t1940 * t890 + F::cast_from(2.0_f64) * t18871 * t1940 * t890 - F::cast_from(6.0_f64) * t18875 * t2403 * t4556 + F::cast_from(12.0_f64) * t4433 * t4541 * t4546 + t10592 - t10596 - t10604 - t10611 - t14618 + t18571 + t18572 - t18573 - t18574 + t18578 + t18579 + t18581 + t18582 - t9524 + t9542;
    t18882
}
