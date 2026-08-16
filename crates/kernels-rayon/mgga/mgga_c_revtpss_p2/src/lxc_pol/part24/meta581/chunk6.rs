//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1811/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1811(t91810: f64, t91824: f64, t543: f64, t6816: f64, t6836: f64, t13804: f64, t1410: f64, t1868: f64, t22046: f64, t22079: f64, t22893: f64, t3934: f64, t3936: f64, t4003: f64, t4012: f64, t46627: f64, t5671: f64, t6869: f64, t73778: f64, t73789: f64, t828: f64, t85514: f64, t85516: f64, t85532: f64, t85543: f64, t85545: f64, t85553: f64, t85609: f64, t85648: f64, t85652: f64, t9955: f64, t9994: f64) -> (f64, f64, f64, f64, f64) {
    let t91826 = t91810 / 2.0_f64 + t91824 / 2.0_f64;
    let t91865 = t543 * t6816;
    let t91870 = t6836 * t6836;
    let t91875 = t6816 * t6816;
    let t91882 = -0.30492001685571196936e-2_f64 * t85514 - 0.48018900292238105408e-1_f64 * t85516 - 0.25724410870841842184e-1_f64 * t3934 * t9955 * t22046 * t22893 - 0.12196800674228478774e-2_f64 * t85532 - 0.15246000842785598467e-3_f64 * t85543 + 0.24009450146119052704e0_f64 * t85545 + 0.27210710165601593064e0_f64 * t73778 - 0.65049603595885220128e-2_f64 * t73789 - 0.25724410870841842184e-1_f64 * t3934 * t9955 * t22079 * t22893 + 0.20579528696673473747e-1_f64 * t13804 * t3936 * t85553 * t9994 * t1868 + 0.51448821741683684368e-1_f64 * t5671 * t9955 * t22046 * t4003 * t6836 + 0.34299214494455789577e-2_f64 * t3934 * t3936 * t85609 * t6869 + 0.51448821741683684366e-2_f64 * t3934 * t3936 * t22079 * t91865 + 0.18007087609589289528e0_f64 * t1410 * t46627 * t828 * t91870 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t91875 + 0.60984003371142393869e-3_f64 * t85648 - 0.48018900292238105408e-1_f64 * t85652;
    (t91826, t91865, t91870, t91875, t91882)
}
