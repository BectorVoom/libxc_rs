//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 831/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk831(t41689: f64, t34264: f64, t7030: f64, t10177: f64, t10523: f64, t544: f64, t899: f64, t913: f64, t12957: f64, t1441: f64, t39968: f64, t12939: f64, t1407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41690 = 0.17041300423964777634e0_f64 * t41689;
    let t41691 = t34264 * t7030;
    let t41692 = 0.29792074959875355558e-1_f64 * t41691;
    let t41696 = t544 * t10523 * t899 * t913 * t10177;
    let t41697 = 0.17875244975925213335e0_f64 * t41696;
    let t41698 = t1441 * t12957;
    let t41699 = 0.1022478025437886658e1_f64 * t41698;
    let t41700 = 0.19171462976960374838e1_f64 * t39968;
    let t41705 = t1407 * t12939;
    (t41690, t41692, t41697, t41699, t41700, t41705)
}
