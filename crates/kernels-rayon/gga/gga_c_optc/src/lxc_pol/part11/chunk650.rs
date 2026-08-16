//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 650/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk650(t450: f64, t5313: f64, t1111: f64, t1121: f64, t1133: f64, t1503: f64, t1520: f64, t431: f64, t4310: f64, t4334: f64, t4369: f64, t4381: f64, t451: f64, t5276: f64, t5280: f64, t5286: f64, t5290: f64, t5298: f64, t5302: f64) -> (f64, f64) {
    let t5314 = t450 * t5313;
    let t5317 = 11.0_f64 / 108.0_f64 * t5276 * t431 + 0.9176114905888133291e-1_f64 * t5280 * t451 - t4310 * t1503 / 54.0_f64 + t1111 * t5286 / 288.0_f64 + t1111 * t5290 / 216.0_f64 + 0.24147670804968771818e-2_f64 * t4381 + 0.47333755318775392234e-1_f64 * t4334 - 0.19318136643975017455e-1_f64 * t4369 * t1520 + 0.18110753103726578864e-2_f64 * t1133 * t5298 + 0.30184588506210964773e-2_f64 * t1133 * t5302 + 0.35500316489081544176e-1_f64 * t1121 * t5314;
    (t5314, t5317)
}
