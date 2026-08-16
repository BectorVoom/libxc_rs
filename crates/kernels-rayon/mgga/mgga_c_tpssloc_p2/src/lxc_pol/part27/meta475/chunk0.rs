//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1844/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844(t3014: f64, t343: f64, t6734: f64, t1004: f64, t6758: f64, t1036: f64, t6750: f64, t1940: f64, t3087: f64, t354: f64, t6759: f64, t3: f64, t6740: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23547 = t3014 * t343;
    let t23548 = t23547 * t6734;
    let t23551 = t1004 * t6758;
    let t23554 = t6750 * t1036;
    let t23556 = t1940 * t3087;
    let t23557 = t354 * t23556;
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562)
}
