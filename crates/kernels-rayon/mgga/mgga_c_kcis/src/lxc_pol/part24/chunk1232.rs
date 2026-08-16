//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1232/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1232(t100090: f64, t100094: f64, t26966: f64, t27014: f64, t28102: f64, t29104: f64, t29108: f64, t7772: f64, t8091: f64, t92600: f64, t96728: f64, t96763: f64, t96779: f64, t96926: f64, t97267: f64) -> f64 {
    let t100102 = 0.30918233506944444445e-4_f64 * t96926 * t28102 + 0.25742669753086419753e-4_f64 * t92600 - 0.61890573922526041666e-5_f64 * t96728 + t96763 - 0.23168402777777777778e-3_f64 * t97267 * t8091 - 0.46377350260416666667e-4_f64 * t7772 * t100090 + 0.15476481481481481481e-2_f64 * t100094 + 0.30891203703703703704e-3_f64 * t26966 * t29108 + 0.23168402777777777778e-3_f64 * t27014 * t29104 + t96779 - 0.11584201388888888889e-3_f64 * t27014 * t29108;
    t100102
}
