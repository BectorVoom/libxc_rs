//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1299/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1299<F: Float>(t11200: F, t695: F, t2063: F, t4824: F, t7242: F, t32955: F, t34122: F, t116477: F, t9649: F, t112269: F, t112406: F, t112709: F, t116448: F, t116453: F, t116533: F, t15909: F, t1693: F, t17345: F, t20: F, t2454: F, t2785: F, t32942: F, t32990: F, t33031: F, t33056: F, t34013: F, t34027: F, t34037: F, t34148: F, t4982: F) -> (F, F) {
    let t116581 = t11200 * t695;
    let t116584 = t7242 * t116581 * t2063 * t4824;
    let t116599 = t34122 * t32955;
    let t116601 = t9649 * t116477;
    let t116607 = 0.27777777777777777779e-1 * t1693 * t4982 * t2454 * t20 * t2785 + 0.26805555555555555556e-2 * t112709 * t34013 + 0.13402777777777777778e-2 * t33056 * t116448 - 0.77602083333333333335e-3 * t112406 * t116584 - 0.53611111111111111112e-2 * t33056 * t116453 - 0.27777777777777777778e-1 * t33031 * t17345 * t34037 * t15909 + 0.26805555555555555556e-2 * t112269 * t34027 + 0.26805555555555555556e-2 * t112709 * t34027 + 0.26805555555555555556e-2 * t33056 * t116533 - 0.23148148148148148148e-2 * t116599 - 0.80416666666666666667e-2 * t116601 - 0.20833333333333333334e-1 * t32942 * t34148 - 0.20833333333333333334e-1 * t32990 * t34148;
    (t116584, t116607)
}
