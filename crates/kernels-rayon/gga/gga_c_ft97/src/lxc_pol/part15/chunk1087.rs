//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1087/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1087(t1060: f64, t12680: f64, t12709: f64, t12968: f64, t12969: f64, t17164: f64, t1901: f64, t20655: f64, t20723: f64, t20763: f64, t20858: f64, t20926: f64, t2210: f64, t2221: f64, t2992: f64, t41251: f64, t446: f64, t574: f64, t63746: f64, t63795: f64, t76567: f64, t77602: f64, t77713: f64, t9144: f64, t925: f64) -> f64 {
    let t87589 = 16.0_f64 / 9.0_f64 * t63746 - 8.0_f64 / 9.0_f64 * t1901 * t17164 * t20858 + 4.0_f64 / 9.0_f64 * t1901 * t2221 * t76567 * t925 - 4.0_f64 / 3.0_f64 * t1901 * t9144 * t20723 * t925 - 8.0_f64 / 3.0_f64 * t1901 * t12709 * t2992 * t20763 - 4.0_f64 / 3.0_f64 * t446 * t574 * t1060 * t20655 + 4.0_f64 / 9.0_f64 * t1901 * t2210 * t77602 * t925 + 4.0_f64 / 3.0_f64 * t1901 * t12680 * t20926 - 8.0_f64 * t1901 * t12968 * t12969 * t20723 + 8.0_f64 / 3.0_f64 * t1901 * t41251 * t77713 * t925 - 16.0_f64 / 9.0_f64 * t63795;
    t87589
}
