//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1332/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1332<F: Float>(t12345: F, t6048: F, t8186: F, t1555: F, t29430: F, t101839: F, t101840: F, t101841: F, t102813: F, t102816: F, t102820: F, t102823: F, t102828: F, t102833: F, t12933: F, t12940: F, t1636: F, t17710: F, t18268: F, t23268: F, t23373: F, t27702: F, t28655: F, t28666: F, t29489: F, t51097: F, t7998: F, t8251: F) -> (F, F, F) {
    let t102836 = F::cast_from(12.0_f64) * t12345 * t8186 * t6048;
    let t102839 = F::cast_from(6.0_f64) * t12345 * t29430 * t1555;
    let t102840 = -F::cast_from(6.0_f64) * t12940 * t1636 * t29489 - t102823 * t1636 + F::cast_from(2.0_f64) * t12933 * t29489 - F::cast_from(2.0_f64) * t17710 * t8251 + F::cast_from(4.0_f64) * t18268 * t28666 + F::cast_from(4.0_f64) * t23268 * t27702 - t23373 * t7998 - F::cast_from(12.0_f64) * t28655 * t51097 + t101839 + t101840 + t101841 - t102813 - t102816 - t102820 - t102828 - t102833 + t102836 + t102839;
    (t102836, t102839, t102840)
}
