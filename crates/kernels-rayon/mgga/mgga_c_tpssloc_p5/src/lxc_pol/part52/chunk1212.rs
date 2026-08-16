//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1212/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1212(t218: f64, t32849: f64, t1528: f64, t1912: f64, t25188: f64, t25348: f64, t259: f64, t30655: f64, t30662: f64, t30741: f64, t30748: f64, t32865: f64, t32869: f64, t32877: f64, t32878: f64, t4147: f64, t6627: f64, t7538: f64, t8363: f64) -> (f64, f64) {
    let t32880 = t218 * t32849;
    let t32884 = -t1528 * t30741 - 2.0_f64 * t1912 * t25188 - 2.0_f64 * t1912 * t25348 + t259 * t32878 + t259 * t32880 - t4147 * t8363 - 2.0_f64 * t6627 * t7538 - t30655 + t30662 + t30748 + t32865 - t32869 - t32877;
    (t32880, t32884)
}
