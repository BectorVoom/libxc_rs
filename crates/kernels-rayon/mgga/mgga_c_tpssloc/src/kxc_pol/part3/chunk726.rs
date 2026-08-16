//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 726/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk726(t1285: f64, t588: f64, t1287: f64, t2423: f64, t3686: f64, t3697: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3828: f64, t3830: f64, t3832: f64) -> (f64, f64, f64, f64) {
    let t3833 = t588 * t1285;
    let t3834 = 8.0_f64 * t3833;
    let t3836 = 8.0_f64 * t588 * t1287;
    let t3837 = t3686 + t3819 + t3821 - t3823 - t2423 + t3825 + t3697 + t3828 - t3830 - t3832 + t3834 + t3836;
    (t3833, t3834, t3836, t3837)
}
