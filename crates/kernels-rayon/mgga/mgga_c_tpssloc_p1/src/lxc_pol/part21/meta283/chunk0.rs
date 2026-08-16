//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1573/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1573(t10236: f64, t2244: f64, t2987: f64, t3008: f64, t2250: f64, t2989: f64, t2775: f64, t343: f64) -> (f64, f64, f64, f64) {
    let t10237 = t10236 * t2244;
    let t10241 = t2987 * t3008;
    let t10245 = t2989 * t2250;
    let t10254 = t343 * t2775;
    (t10237, t10241, t10245, t10254)
}
