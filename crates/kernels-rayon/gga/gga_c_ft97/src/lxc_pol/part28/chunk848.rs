//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 848/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk848(t34524: f64, t34534: f64, t103: f64, t5710: f64, t6557: f64, t1332: f64, t26061: f64, t32545: f64, t979: f64, t1286: f64, t31997: f64, t32000: f64, t32025: f64, t32401: f64, t34354: f64, t34358: f64, t34362: f64, t34366: f64, t34368: f64, t34512: f64, t34514: f64, t6414: f64, t6423: f64, t6461: f64, t7162: f64, t7168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34535 = t34524 + t34534;
    let t34536 = t34535 * t103;
    let t34542 = t5710 * t6557;
    let t34544 = t26061 * t1332;
    let t34546 = t32545 * t979;
    let t34548 = -t31997 - t32000 - t32025 - t7162 * t6423 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t34354 - t1286 * t34358 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t34362 + t1286 * t34366 - 2.0_f64 * t34368 - 2.0_f64 * t34512 + 4.0_f64 * t34514 + 2.0_f64 * t34536 + t32401 - t6414 * t7168 / 3.0_f64 + t7162 * t6461 / 6.0_f64 - 4.0_f64 * t34542 - 4.0_f64 * t34544 - 2.0_f64 * t34546;
    (t34535, t34536, t34542, t34544, t34546, t34548)
}
