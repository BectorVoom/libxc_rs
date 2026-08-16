//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1237/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1237(t38617: f64, t40070: f64, t41668: f64, t41669: f64, t41670: f64, t41671: f64, t41672: f64, t41682: f64, t41687: f64, t41689: f64, t43488: f64, t43490: f64) -> f64 {
    let t44412 = t41668 + t41669 + t41670 + t41671 - t41672 + 0.34672886960217074252e0_f64 * t43488 - 0.23804984598836975487e0_f64 * t40070 + t41682 - t38617 + 0.19514881078765566037e-1_f64 * t43490 - t41687 + t41689;
    t44412
}
