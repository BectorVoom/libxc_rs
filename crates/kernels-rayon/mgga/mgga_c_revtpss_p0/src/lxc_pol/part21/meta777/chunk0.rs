//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2768/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2768(t50880: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t50857: f64, t50861: f64, t50864: f64, t50866: f64, t50869: f64, t50871: f64, t50872: f64, t50874: f64, t50875: f64, t50876: f64, t50879: f64) -> (f64, f64) {
    let t50881 = 72.0_f64 * t50880;
    let t50882 = -t50857 + t50861 + t50864 + t50866 + t50869 + t50871 - t50872 + t40067 - t40072 + t50874 + t40167 - t40171 - t50875 + t50876 - t40184 + t50879 + t50881;
    (t50881, t50882)
}
