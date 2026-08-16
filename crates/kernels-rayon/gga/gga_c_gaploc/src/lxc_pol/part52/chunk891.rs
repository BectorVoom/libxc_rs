//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 891/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk891(t13658: f64, t2197: f64, t13585: f64, t5559: f64, t841: f64, t13578: f64, t14537: f64, t11305: f64, t6556: f64, t3599: f64, t6553: f64, t2595: f64, t36313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45958 = 0.43710935587469654631e2_f64 * t2197 * t13658;
    let t45967 = 6.0_f64 * t5559 * t13585 * t841;
    let t45969 = 6.0_f64 * t14537 * t13578;
    let t45973 = 2.0_f64 * t6556 * t11305;
    let t45974 = t6553 * t3599;
    let t45976 = 2.0_f64 * t36313 * t2595;
    (t45958, t45967, t45969, t45973, t45974, t45976)
}
