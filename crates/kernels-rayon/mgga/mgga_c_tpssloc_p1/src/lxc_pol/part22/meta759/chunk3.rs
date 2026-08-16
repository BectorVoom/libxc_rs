//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2552/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2552(t71183: f64, t71187: f64, t71446: f64, t71449: f64, t71452: f64, t71454: f64, t71456: f64, t71458: f64, t71461: f64, t71463: f64, t71465: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64, t71486: f64, t71489: f64) -> (f64, f64) {
    let t71611 = -0.59793333333333333333e0_f64 * t71183 - 0.59793333333333333333e0_f64 * t71187 + 0.15358125e0_f64 * t71446 - 0.9494625e0_f64 * t71449 - 0.3560484375e1_f64 * t71452 + 0.427258125e1_f64 * t71454 - 0.28483875e1_f64 * t71456 - 0.28483875e1_f64 * t71458 + 0.1151859375e0_f64 * t71461 - 0.230371875e0_f64 * t71463 + 0.46074375e0_f64 * t71465;
    let t71624 = 0.46074375e0_f64 * t71468 - 0.2434271604938271605e-1_f64 * t71470 + 0.10954222222222222222e0_f64 * t71472 - 0.32862666666666666666e0_f64 * t71474 + 0.16431333333333333333e0_f64 * t71477 - 0.82156666666666666668e-1_f64 * t71480 - 0.82156666666666666668e-1_f64 * t71483 + 0.49293999999999999999e0_f64 * t71486 + 0.49293999999999999999e0_f64 * t71489 + 0.17938e1_f64 * t71191 - 0.35876000000000000001e1_f64 * t71195 - 0.71752000000000000002e1_f64 * t71199;
    (t71611, t71624)
}
