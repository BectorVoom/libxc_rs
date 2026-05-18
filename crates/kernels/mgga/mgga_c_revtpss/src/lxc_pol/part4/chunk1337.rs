//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1337/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1337<F: Float>(t16807: F, t422: F, t12552: F, t1756: F, t12555: F, t3497: F, t1196: F, t16708: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12367: F, t16706: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F) {
    let t16809 = F::new(0.621814e-1) * t16807 * t422;
    let t16810 = t12552 * t1756;
    let t16811 = t12555 * t3497;
    let t16812 = t16810 * t16811;
    let t16814 = F::new(0.10254018858216406658e4) * t1196 * t16812;
    let t16820 = F::new(0.41203703703703703704e-2) * t16708;
    let t16821 = F::new(0.12361111111111111111e-1) * t16710;
    let t16822 = F::new(0.61805555555555555556e-2) * t16712;
    let t16831 = -t12367 + F::new(0.82407407407407407407e-2) * t12297 + F::new(0.20601851851851851852e-2) * t12299 - F::new(0.61805555555555555556e-2) * t12301 - F::new(0.30902777777777777778e-2) * t12303 + F::new(0.41203703703703703704e-2) * t16706 + t16820 - t16821 - t16822 + F::new(0.10300925925925925926e-1) * t16717 - F::new(0.37083333333333333333e-1) * t16722 - F::new(0.12361111111111111111e-1) * t16727 - F::new(0.61805555555555555555e-2) * t16731 + F::new(0.55625000000000000001e-1) * t16735 + F::new(0.37083333333333333334e-1) * t16740 + F::new(0.18541666666666666667e-1) * t16744 + F::new(0.92708333333333333333e-2) * t16748;
    (t16809, t16814, t16831)
}
