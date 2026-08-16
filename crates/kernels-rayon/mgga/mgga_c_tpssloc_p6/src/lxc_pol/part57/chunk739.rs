//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 739/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk739(t22751: f64, t7733: f64, t22893: f64, t7732: f64, t22892: f64, t1834: f64, t552: f64, t1824: f64, t2006: f64, t6914: f64, t7737: f64, t1799: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26381 = t22751 * t7733;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26403 = t2006 * t1824;
    let t26406 = t6914 * t7737;
    let t26421 = t562 * t1799;
    (t26381, t26393, t26395, t26403, t26406, t26421)
}
