//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1232/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1232<F: Float>(t100090: F, t100094: F, t26966: F, t27014: F, t28102: F, t29104: F, t29108: F, t7772: F, t8091: F, t92600: F, t96728: F, t96763: F, t96779: F, t96926: F, t97267: F) -> F {
    let t100102 = F::new(0.30918233506944444445e-4) * t96926 * t28102 + F::new(0.25742669753086419753e-4) * t92600 - F::new(0.61890573922526041666e-5) * t96728 + t96763 - F::new(0.23168402777777777778e-3) * t97267 * t8091 - F::new(0.46377350260416666667e-4) * t7772 * t100090 + F::new(0.15476481481481481481e-2) * t100094 + F::new(0.30891203703703703704e-3) * t26966 * t29108 + F::new(0.23168402777777777778e-3) * t27014 * t29104 + t96779 - F::new(0.11584201388888888889e-3) * t27014 * t29108;
    t100102
}
