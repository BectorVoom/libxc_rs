//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2193/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2193<F: Float>(t870: F, t87944: F, t1877: F, t1915: F, t22959: F, t23290: F, t25: F, t25013: F, t25021: F, t25024: F, t2522: F, t25377: F, t25381: F, t25392: F, t4314: F, t6666: F, t6670: F, t6671: F, t81483: F, t86803: F, t86806: F, t86810: F, t86816: F, t86821: F, t86825: F, t86830: F, t86835: F, t86836: F) -> (F, F) {
    let t87945 = t87944 * t870;
    let t87952 = -F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t86803 - t1877 * t6670 * t86806 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t22959 * t86810 - F::cast_from(3.0_f64) * t81483 * t25021 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t86816 - t1877 * t23290 * t25381 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t86821 + F::cast_from(3.0_f64) * t4314 * t1915 * t86825 + F::cast_from(6.0_f64) * t25013 * t86830 + t86835 - t1877 * t86836 * t6671 - t1877 * t23290 * t25392 - t1877 * t23290 * t25377 + t1877 * t87945 * t25 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t6666 * t25024;
    (t87945, t87952)
}
