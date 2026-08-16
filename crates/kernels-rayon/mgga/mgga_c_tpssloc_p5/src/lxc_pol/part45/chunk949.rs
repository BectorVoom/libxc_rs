//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 949/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk949(t114016: f64, t12368: f64, t3792: f64, t5248: f64, t31170: f64, t550: f64, t22824: f64, t31159: f64, t22866: f64, t8462: f64, t1307: f64, t22690: f64, t22792: f64, t6950: f64) -> (f64, f64, f64, f64, f64) {
    let t114019 = t114016 * t5248 * t12368 * t3792;
    let t114023 = t31170 * t5248 * t12368 * t550;
    let t114025 = t22824 * t31159;
    let t114027 = t22866 * t8462;
    let t114031 = t22792 * t22690 * t6950 * t1307;
    (t114019, t114023, t114025, t114027, t114031)
}
