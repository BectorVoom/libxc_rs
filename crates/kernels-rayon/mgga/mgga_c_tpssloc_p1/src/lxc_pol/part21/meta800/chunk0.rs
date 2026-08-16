//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2787/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2787(t118: f64, t2375: f64, t5522: f64, t46335: f64, t46348: f64, t16575: f64, t706: f64, t708: f64, t46369: f64, t46371: f64, t39549: f64, t39563: f64, t39585: f64, t39590: f64, t39593: f64, t40801: f64, t40803: f64, t58060: f64, t58061: f64, t58062: f64, t58080: f64, t58085: f64, t58094: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58972 = t5522 * t118 * t2375;
    let t58973 = 0.10843581300301739842e-1_f64 * t58972;
    let t58974 = 16.0_f64 * t46335;
    let t58975 = 48.0_f64 * t46348;
    let t58976 = t706 * t16575;
    let t58978 = 8.0_f64 * t58976 * t708;
    let t58979 = 8.0_f64 * t46369;
    let t58980 = 0.43374325201206959368e-1_f64 * t46371;
    let t58981 = t40801 - t40803 - t58060 + t58061 + t58062 + t39549 + t58080 + t39563 + t58085 - t58094 + t58973 - t39585 + t39590 + t58974 - t39593 + t58975 + t58978 + t58979 - t58980;
    (t58973, t58974, t58975, t58978, t58979, t58980, t58981)
}
