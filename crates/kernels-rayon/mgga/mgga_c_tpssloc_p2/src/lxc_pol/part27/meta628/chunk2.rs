//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2115/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115(t1877: f64, t22959: f64, t22961: f64, t25013: f64, t25015: f64, t2522: f64, t25366: f64, t25372: f64, t25375: f64, t25385: f64, t6666: f64, t6670: f64, t81483: f64, t86703: f64, t86707: f64, t86710: f64, t86714: f64, t86718: f64, t86722: f64, t86727: f64, t86734: f64, t86736: f64, t86740: f64, t86746: f64, t86751: f64) -> f64 {
    let t86752 = -t1877 * t6670 * t86746 + 3.0_f64 * t2522 * t25385 * t6666 - 3.0_f64 * t22959 * t86710 - 3.0_f64 * t22959 * t86722 - 3.0_f64 * t22959 * t86727 - 3.0_f64 * t22961 * t86736 - 3.0_f64 * t25013 * t86707 + 6.0_f64 * t25015 * t86740 - 3.0_f64 * t25366 * t81483 + t25372 * t86714 - 3.0_f64 * t25372 * t86718 + 2.0_f64 * t25375 * t86703 - t86734 - t86751;
    t86752
}
