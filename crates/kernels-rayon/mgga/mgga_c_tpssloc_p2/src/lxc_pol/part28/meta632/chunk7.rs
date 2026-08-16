//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1996/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1996(t870: f64, t92989: f64, t10143: f64, t7844: f64, t1877: f64, t2057: f64, t22964: f64, t23296: f64, t24191: f64, t25: f64, t2522: f64, t25385: f64, t26563: f64, t26740: f64, t26756: f64, t6542: f64, t7110: f64, t7114: f64, t7845: f64, t86718: f64, t86722: f64, t86798: f64, t86821: f64, t87984: f64, t87998: f64, t92356: f64, t92359: f64, t92362: f64, t92364: f64) -> (f64, f64, f64) {
    let t92990 = t92989 * t870;
    let t93000 = t7844 * t10143;
    let t93005 = -3.0_f64 * t26756 * t86718 - t1877 * t7114 * t87984 / 2.0_f64 + t92356 - t92359 + t92362 - t92364 + 3.0_f64 * t2522 * t26740 * t6542 - 6.0_f64 * t26563 * t86798 - 3.0_f64 * t24191 * t87998 + 3.0_f64 * t2522 * t7845 * t22964 + t1877 * t92990 * t25 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t86821 + 3.0_f64 * t2522 * t7110 * t25385 + t1877 * t93000 * t23296 - 3.0_f64 * t24191 * t86722;
    (t92990, t93000, t93005)
}
