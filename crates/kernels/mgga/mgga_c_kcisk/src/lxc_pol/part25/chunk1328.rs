//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1328/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1328<F: Float>(t16981: F, t415: F, t9687: F, t11236: F, t1869: F, t34160: F, t654: F, t33001: F, t7218: F, t112289: F, t112387: F, t112724: F, t112726: F, t112739: F, t116925: F, t116939: F, t1636: F, t1791: F, t32990: F, t33005: F, t33023: F, t33031: F, t34021: F, t34023: F, t34122: F, t34218: F, t4648: F, t5015: F, t7268: F, t9936: F) -> (F, F, F) {
    let t117237 = t415 * t9687 * t16981;
    let t117246 = t1869 * t11236 * t654 * t34160;
    let t117248 = t33001 * t7218;
    let t117252 = 0.69444444444444444446e-2 * t112289 * t34023 + 0.69444444444444444446e-2 * t33031 * t5015 * t1791 * t7268 * t1636 + 0.34722222222222222223e-2 * t33031 * t5015 * t34021 * t4648 - 0.69444444444444444446e-2 * t33031 * t116939 - 0.13888888888888888889e-1 * t33031 * t116925 - 0.34722222222222222223e-2 * t112387 * t9936 + 0.33163888888888888888e-2 * t112724 - 0.24872916666666666666e-2 * t117237 - 0.3684876543209876543e-3 * t112726 - 0.20833333333333333334e-1 * t34122 * t33023 + 0.20833333333333333334e-1 * t32990 * t34218 + 0.99491666666666666664e-2 * t117246 + 0.62081666666666666671e-2 * t117248 * t33005 - 0.33163888888888888888e-2 * t112739;
    (t117237, t117246, t117252)
}
