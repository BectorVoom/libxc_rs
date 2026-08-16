//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2162/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2162(t870: f64, t87944: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25: f64, t25013: f64, t25021: f64, t25024: f64, t2522: f64, t25377: f64, t25381: f64, t25392: f64, t4314: f64, t6666: f64, t6670: f64, t6671: f64, t81483: f64, t86803: f64, t86806: f64, t86810: f64, t86816: f64, t86821: f64, t86825: f64, t86830: f64, t86835: f64, t86836: f64) -> (f64, f64) {
    let t87945 = t87944 * t870;
    let t87952 = -3.0_f64 / 2.0_f64 * t22959 * t86803 - t1877 * t6670 * t86806 / 2.0_f64 - 3.0_f64 * t22959 * t86810 - 3.0_f64 * t81483 * t25021 - 3.0_f64 / 2.0_f64 * t22959 * t86816 - t1877 * t23290 * t25381 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t86821 + 3.0_f64 * t4314 * t1915 * t86825 + 6.0_f64 * t25013 * t86830 + t86835 - t1877 * t86836 * t6671 - t1877 * t23290 * t25392 - t1877 * t23290 * t25377 + t1877 * t87945 * t25 / 2.0_f64 + 3.0_f64 * t2522 * t6666 * t25024;
    (t87945, t87952)
}
