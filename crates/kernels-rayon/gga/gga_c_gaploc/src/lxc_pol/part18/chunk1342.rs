//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1342/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1342(t1382: f64, t8435: f64, t921: f64, t2902: f64, t6553: f64, t24295: f64, t2595: f64, t11135: f64, t5559: f64, t841: f64, t24282: f64, t7324: f64, t8859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33966 = 2.0_f64 * t1382 * t8435 * t921;
    let t33968 = 2.0_f64 * t6553 * t2902;
    let t33970 = 4.0_f64 * t24295 * t2595;
    let t33973 = 12.0_f64 * t5559 * t11135 * t841;
    let t33974 = t24282 * t921;
    let t33977 = 4.0_f64 * t7324 * t8859;
    (t33966, t33968, t33970, t33973, t33974, t33977)
}
