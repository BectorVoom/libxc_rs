//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1233/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1233<F: Float>(t22632: F, t30071: F, t5813: F, t422: F, t4710: F, t4702: F, t4698: F, t53: F, t5591: F, t72: F, t100634: F, t1013: F, t104690: F, t104712: F, t104735: F, t104782: F, t115977: F, t118744: F, t1570: F, t23705: F, t23832: F, t26692: F, t26695: F, t26705: F, t3052: F, t3188: F, t379: F, t5570: F, t925: F, t93169: F, t94434: F, t94508: F) -> (F, F) {
    let t118753 = t5813 * t22632 * t30071;
    let t118758 = t422 * t4710;
    let t118782 = t422 * t4702;
    let t118789 = t5591 * t72 * t4698 * t53;
    let t118792 = -0.33339000546296296297e-1 * t118753 + 0.18122740165211489339e1 * t104690 * t118744 - 0.4445200072839506173e-1 * t104712 + 0.33339000546296296297e-1 * t23705 * t5570 * t118758 * t379 + 0.66678001092592592595e-1 * t23705 * t5570 * t104782 * t925 + 0.13335600218518518519e0 * t23705 * t100634 * t26695 * t3052 + 0.13335600218518518519e0 * t94434 * t93169 * t104735 * t26705 - 0.13335600218518518519e0 * t23705 * t93169 * t1013 * t1570 * t3188 - 0.8890400145679012346e-1 * t26692 * t115977 - 0.10001700163888888889e0 * t94508 * t5570 * t118782 * t379 + 0.24167761770734866964e0 * t23832 * t118789;
    (t118789, t118792)
}
