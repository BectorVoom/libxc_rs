//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1969/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969<F: Float>(t193: F, t201: F, t7844: F, t1877: F, t2057: F, t2249: F, t22951: F, t22961: F, t22968: F, t23299: F, t24191: F, t25024: F, t2522: F, t25366: F, t26563: F, t26744: F, t4314: F, t7110: F, t7114: F, t7845: F, t84797: F, t86710: F, t86746: F, t86782: F, t86803: F, t86816: F, t86825: F, t87981: F, t87994: F) -> (F, F) {
    let t92319 = t193 * t201 * t7844;
    let t92349 = -F::cast_from(3.0_f64) * t24191 * t86710 + F::cast_from(3.0_f64) * t2522 * t7110 * t25024 - F::cast_from(3.0_f64) * t84797 * t25366 - F::cast_from(3.0_f64) * t92319 * t22961 - t1877 * t7114 * t87994 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t86816 + F::cast_from(3.0_f64) * t4314 * t7845 * t22951 + F::cast_from(3.0_f64) * t4314 * t2057 * t86825 + t1877 * t7845 * t2249 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t86803 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7845 * t22968 + F::cast_from(6.0_f64) * t24191 * t86782 - t1877 * t7114 * t86746 - t1877 * t26744 * t23299 + F::cast_from(3.0_f64) * t26563 * t87981;
    (t92319, t92349)
}
