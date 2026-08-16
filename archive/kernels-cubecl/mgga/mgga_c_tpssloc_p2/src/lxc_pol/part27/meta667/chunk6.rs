//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2349/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2349<F: Float>(t1985: F, t6907: F, t90739: F, t22685: F, t22686: F, t26193: F, t16018: F, t6888: F, t6889: F, t6890: F, t22674: F, t22892: F, t26189: F) -> (F, F, F, F) {
    let t91469 = t1985 * t90739 * t6907;
    let t91478 = t22685 * t26193 * t22686;
    let t91482 = t6888 * t6889 * t6890 * t16018;
    let t91486 = t22892 * t22674 * t26189;
    (t91469, t91478, t91482, t91486)
}
