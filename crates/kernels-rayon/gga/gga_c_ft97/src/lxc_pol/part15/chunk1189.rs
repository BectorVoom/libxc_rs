//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1189/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1189(t10261: f64, t10613: f64, t192: f64, t19709: f64, t22161: f64, t2681: f64, t2766: f64, t2771: f64, t2781: f64, t4199: f64, t4206: f64, t4218: f64, t43803: f64, t462: f64, t5299: f64, t83569: f64, t83587: f64, t848: f64, t852: f64, t88149: f64, t88153: f64, t88184: f64, t88240: f64, t88248: f64, t89770: f64, t89889: f64, t90304: f64, t90308: f64, t90313: f64, t92: f64) -> f64 {
    let t90464 = 4.0_f64 / 3.0_f64 * t462 * t2771 * t89770 + 8.0_f64 / 3.0_f64 * t462 * t4206 * t88149 - 8.0_f64 / 9.0_f64 * t462 * t4199 * t88153 + 8.0_f64 * t462 * t4199 * t88184 - 16.0_f64 / 3.0_f64 * t462 * t10613 * t89889 + 8.0_f64 * t462 * t2681 * t4218 * t22161 - 2.0_f64 / 3.0_f64 * t462 * t2766 * t88240 - t462 * t848 * t88248 / 3.0_f64 - 36.0_f64 * t462 * t10261 * t19709 * t5299 - 16.0_f64 / 9.0_f64 * t83569 + 6.0_f64 * t92 * t192 * t2781 * t90313 + 24.0_f64 * t92 * t192 * t43803 * t90308 - t92 * t192 * t852 * t90304 - 8.0_f64 * t83587;
    t90464
}
