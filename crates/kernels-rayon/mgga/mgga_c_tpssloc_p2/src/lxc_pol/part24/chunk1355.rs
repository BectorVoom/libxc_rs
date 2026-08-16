//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1355/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1355(t6553: f64, t6554: f64, t81984: f64, t9458: f64, t225: f64, t23211: f64, t23205: f64, t82038: f64, t23242: f64, t81979: f64, t10049: f64, t10104: f64, t1912: f64, t218: f64, t23191: f64, t259: f64, t2591: f64, t2597: f64, t40875: f64, t6624: f64, t6627: f64, t6632: f64, t6663: f64, t81976: f64, t866: f64) -> f64 {
    let t82282 = t81984 * t6553 * t6554 * t9458;
    let t82287 = t23211 * t225;
    let t82294 = t82038 * t23205;
    let t82296 = t81979 * t23242;
    let t82304 = -0.19739208802178717238e0_f64 * t82282 - t6627 * t10104 + t218 * t81976 * t259 - 6.0_f64 * t82287 * t866 + 6.0_f64 * t10049 * t6632 - 3.0_f64 * t2597 * t23191 - 0.15626873635058151147e0_f64 * t82294 - 0.34543615403812755166e0_f64 * t82296 + 3.0_f64 * t2591 * t6624 * t259 - 3.0_f64 * t10049 * t6663 - t40875 * t1912;
    t82304
}
