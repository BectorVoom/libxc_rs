//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1198/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1198(t12524: f64, t8657: f64, t20173: f64, t1873: f64, t7056: f64, t3941: f64, t2039: f64, t6534: f64, t23877: f64, t23880: f64, t31284: f64, t31287: f64, t31781: f64, t31795: f64, t31799: f64, t31801: f64, t31803: f64, t577: f64, t671: f64, t7010: f64, t7235: f64, t8508: f64) -> (f64, f64, f64) {
    let t31811 = 27.0_f64 * t12524 * t8657;
    let t31813 = 27.0_f64 * t20173 * t8657;
    let t31814 = t7056 * t1873;
    let t31816 = 27.0_f64 * t3941 * t31814;
    let t31817 = t2039 * t6534;
    let t31819 = 27.0_f64 * t3941 * t31817;
    let t31820 = 0.45e1_f64 * t31781 * t577 + 0.135e2_f64 * t31795 * t671 + t31799 + t31801 + t31803 + 0.135e2_f64 * t23877 * t2039 + 27.0_f64 * t23880 * t7235 + 0.135e2_f64 * t7010 * t7056 + t31811 + t31813 + t31816 + t31819 + t31284 + t31287 + t8508;
    (t31814, t31817, t31820)
}
