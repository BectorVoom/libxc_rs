//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 926/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk926(t24345: f64, t6762: f64, t17806: f64, t17836: f64, t111831: f64, t505: f64, t1091: f64, t3762: f64, t109108: f64, t1100: f64, t2567: f64, t6837: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123543 = t6762 * t24345;
    let t123607 = t17836 * t17806;
    let t123619 = t111831 * t505;
    let t123650 = t1091 * t3762;
    let t123768 = t1100 * t109108;
    let t124402 = t2567 * t6837;
    (t123543, t123607, t123619, t123650, t123768, t124402)
}
