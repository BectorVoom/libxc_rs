//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1227/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1227<F: Float>(t3431: F, t5608: F, t13505: F, t13507: F, t13509: F, t13517: F, t13521: F, t13532: F, t13539: F, t17254: F, t17258: F, t17262: F, t17266: F) -> F {
    let t22455 = t3431 * t5608;
    let t22459 = -F::new(0.12004725073059526352e-1) * t13505 + F::new(0.12004725073059526352e-1) * t13507 - F::new(0.85748036236139473944e-3) * t13509 + F::new(0.40015750243531754508e-2) * t13517 - F::new(0.42874018118069736972e-3) * t13521 - F::new(0.68598428988911579156e-2) * t17254 - F::new(0.34299214494455789578e-2) * t17258 + F::new(0.17149607247227894789e-2) * t17262 - F::new(0.17149607247227894789e-2) * t17266 + F::new(0.32012600194825403606e-1) * t22455 - F::new(0.4801890029223810541e-1) * t13532 + F::new(0.85748036236139473944e-3) * t13539;
    t22459
}
