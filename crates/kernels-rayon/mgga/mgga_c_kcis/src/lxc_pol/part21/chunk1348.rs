//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1348/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1348(t7788: f64, t96812: f64, t95815: f64, t11072: f64, t1646: f64, t26960: f64, t26961: f64, t28102: f64, t3532: f64, t8095: f64, t92590: f64, t92749: f64, t92752: f64, t92785: f64, t93023: f64, t95802: f64, t95805: f64, t95817: f64, t95820: f64) -> f64 {
    let t96875 = t7788 * t96812;
    let t96885 = 0.15476481481481481481e-2_f64 * t95815;
    let t96890 = 0.69644166666666666664e-2_f64 * t95802 - 0.23214722222222222222e-2_f64 * t95805 - 0.20612155671296296296e-4_f64 * t92749 - 0.23168402777777777778e-3_f64 * t92752 - 0.7722800925925925926e-4_f64 * t96875 + 0.23168402777777777778e-3_f64 * t93023 * t28102 - 0.23168402777777777778e-3_f64 * t26960 * t11072 * t26961 * t1646 * t3532 - 0.23168402777777777778e-3_f64 * t92785 - t96885 - 0.51588271604938271604e-3_f64 * t95817 - 0.23214722222222222222e-2_f64 * t95820 + 0.34752604166666666667e-3_f64 * t92590 * t8095;
    t96890
}
