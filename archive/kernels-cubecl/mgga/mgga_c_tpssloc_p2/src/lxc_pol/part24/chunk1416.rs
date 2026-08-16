//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1416/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1416<F: Float>(t12303: F, t24995: F, t8945: F, t113: F, t12504: F, t1976: F, t22483: F, t22619: F, t2314: F, t2363: F, t4034: F, t6517: F, t652: F, t6539: F, t6862: F, t81426: F, t81430: F, t81432: F, t81434: F, t81458: F, t81469: F, t83554: F, t83666: F, t83672: F, t83674: F, t83677: F, t83679: F, t83681: F, t83684: F, t9348: F, t9416: F) -> F {
    let t83687 = F::cast_from(18.0_f64) * t24995 * t8945 * t12303;
    let t83688 = t81426 - F::cast_from(6.0_f64) * t6517 * t12504 - t81430 - t81432 - t81434 - F::cast_from(6.0_f64) * t9348 * t6539 - t81458 - F::cast_from(6.0_f64) * t4034 * t22483 - F::cast_from(6.0_f64) * t652 * t6862 * t2363 - F::cast_from(2.0_f64) * t652 * t1976 * t9416 + t81469 - t113 * (t83554 + t83666) - F::cast_from(12.0_f64) * t2314 * t22619 - t83672 - t83674 - t83677 - t83679 - t83681 - t83684 + t83687;
    t83688
}
