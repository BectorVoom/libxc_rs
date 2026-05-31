//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1811/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811<F: Float>(t91810: F, t91824: F, t543: F, t6816: F, t6836: F, t13804: F, t1410: F, t1868: F, t22046: F, t22079: F, t22893: F, t3934: F, t3936: F, t4003: F, t4012: F, t46627: F, t5671: F, t6869: F, t73778: F, t73789: F, t828: F, t85514: F, t85516: F, t85532: F, t85543: F, t85545: F, t85553: F, t85609: F, t85648: F, t85652: F, t9955: F, t9994: F) -> (F, F, F, F, F) {
    let t91826 = t91810 / F::cast_from(2.0_f64) + t91824 / F::cast_from(2.0_f64);
    let t91865 = t543 * t6816;
    let t91870 = t6836 * t6836;
    let t91875 = t6816 * t6816;
    let t91882 = -F::cast_from(0.30492001685571196936e-2_f64) * t85514 - F::cast_from(0.48018900292238105408e-1_f64) * t85516 - F::cast_from(0.25724410870841842184e-1_f64) * t3934 * t9955 * t22046 * t22893 - F::cast_from(0.12196800674228478774e-2_f64) * t85532 - F::cast_from(0.15246000842785598467e-3_f64) * t85543 + F::cast_from(0.24009450146119052704e0_f64) * t85545 + F::cast_from(0.27210710165601593064e0_f64) * t73778 - F::cast_from(0.65049603595885220128e-2_f64) * t73789 - F::cast_from(0.25724410870841842184e-1_f64) * t3934 * t9955 * t22079 * t22893 + F::cast_from(0.20579528696673473747e-1_f64) * t13804 * t3936 * t85553 * t9994 * t1868 + F::cast_from(0.51448821741683684368e-1_f64) * t5671 * t9955 * t22046 * t4003 * t6836 + F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t85609 * t6869 + F::cast_from(0.51448821741683684366e-2_f64) * t3934 * t3936 * t22079 * t91865 + F::cast_from(0.18007087609589289528e0_f64) * t1410 * t46627 * t828 * t91870 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t91875 + F::cast_from(0.60984003371142393869e-3_f64) * t85648 - F::cast_from(0.48018900292238105408e-1_f64) * t85652;
    (t91826, t91865, t91870, t91875, t91882)
}
