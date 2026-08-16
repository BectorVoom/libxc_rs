//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1720/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1720(t52: f64, t3966: f64, t78: f64, t12606: f64, t1431: f64, t2244: f64, t2250: f64, t4111: f64, t607: f64, t771: f64, t12958: f64, zeta_threshold: f64) -> (f64, f64) {
    let t150 = t52 <= zeta_threshold;
    let t12961 = t78 * t3966;
    let t12969 = piecewise3(t150, 0.0_f64, -8.0_f64 / 27.0_f64 * t1431 * t2244 - 4.0_f64 / 9.0_f64 * t12961 * t607 - 2.0_f64 / 9.0_f64 * t4111 * t2250 - 2.0_f64 / 3.0_f64 * t771 * t12606);
    let t12971 = t12958 / 2.0_f64 + t12969 / 2.0_f64;
    (t12961, t12971)
}
