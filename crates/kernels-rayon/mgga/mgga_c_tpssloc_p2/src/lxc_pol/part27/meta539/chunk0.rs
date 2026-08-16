//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1965/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1965(t1874: f64, t26179: f64, t6525: f64, t7458: f64, t22751: f64, t7692: f64, t22666: f64, t7691: f64, t6888: f64, t5187: f64, t6890: f64, t6889: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26181 = 2.0_f64 * t26179 * t1874;
    let t26183 = 2.0_f64 * t7458 * t6525;
    let t26184 = t22751 * t7692;
    let t26186 = t22666 * t7691;
    let t26187 = t6888 * t26186;
    let t26189 = t6890 * t5187;
    let t26190 = t6889 * t26189;
    (t26181, t26183, t26184, t26186, t26187, t26189, t26190)
}
