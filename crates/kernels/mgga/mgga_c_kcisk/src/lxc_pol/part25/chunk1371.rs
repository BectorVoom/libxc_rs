//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1371/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1371<F: Float>(t34477: F, t9736: F, t34593: F, t9732: F, t34403: F, t5014: F, t118011: F, t33227: F, t113123: F, t116674: F, t116683: F, t116687: F, t116690: F, t116695: F, t116698: F, t116710: F, t116723: F, t117829: F, t117833: F, t33196: F, t9728: F) -> (F, F) {
    let t118098 = 0.34722222222222222222e-2 * t34477 * t9736;
    let t118099 = t34593 * t9732;
    let t118103 = t5014 * t34403;
    let t118105 = t118103 * t118011 * t33227;
    let t118119 = -t118098 + 0.40208333333333333334e-2 * t118099 * t9728 + 0.46429444444444444443e-2 * t116674 - 0.40208333333333333334e-2 * t113123 * t118105 - 0.77382407407407407406e-3 * t116683 - 0.38691203703703703703e-3 * t116687 + 0.19345601851851851852e-2 * t116690 - 0.30952962962962962962e-2 * t116695 - 0.51588271604938271603e-2 * t116698 - 0.11607361111111111111e-2 * t116710 - 0.51588271604938271604e-3 * t116723 + 0.6701388888888888889e-3 * t33196 * t117829 + 0.89351851851851851853e-3 * t33196 * t117833;
    (t118105, t118119)
}
