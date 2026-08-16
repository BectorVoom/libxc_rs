//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1048/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1048(t27254: f64, t7467: f64, t100996: f64, t1873: f64, t2113: f64, t5493: f64, t1458: f64, t7982: f64, t2240: f64, t29473: f64, t8301: f64, t55921: f64, t8662: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128984 = 27.0_f64 * t27254 * t7467;
    let t128988 = 0.135e2_f64 * t100996 * t1873;
    let t129008 = t2113 * t5493;
    let t129015 = t7982 * t1458;
    let t129084 = t2240 * t8301 * t29473;
    let t129093 = t55921 * t8662;
    (t128984, t128988, t129008, t129015, t129084, t129093)
}
