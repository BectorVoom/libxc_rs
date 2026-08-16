//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1332/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1332(t12345: f64, t6048: f64, t8186: f64, t1555: f64, t29430: f64, t101839: f64, t101840: f64, t101841: f64, t102813: f64, t102816: f64, t102820: f64, t102823: f64, t102828: f64, t102833: f64, t12933: f64, t12940: f64, t1636: f64, t17710: f64, t18268: f64, t23268: f64, t23373: f64, t27702: f64, t28655: f64, t28666: f64, t29489: f64, t51097: f64, t7998: f64, t8251: f64) -> (f64, f64, f64) {
    let t102836 = 12.0_f64 * t12345 * t8186 * t6048;
    let t102839 = 6.0_f64 * t12345 * t29430 * t1555;
    let t102840 = -6.0_f64 * t12940 * t1636 * t29489 - t102823 * t1636 + 2.0_f64 * t12933 * t29489 - 2.0_f64 * t17710 * t8251 + 4.0_f64 * t18268 * t28666 + 4.0_f64 * t23268 * t27702 - t23373 * t7998 - 12.0_f64 * t28655 * t51097 + t101839 + t101840 + t101841 - t102813 - t102816 - t102820 - t102828 - t102833 + t102836 + t102839;
    (t102836, t102839, t102840)
}
