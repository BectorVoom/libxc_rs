//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1205/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1205(t91032: f64, t91048: f64, t91065: f64, t91080: f64, t193: f64, t2843: f64, t2862: f64, t295: f64, t312: f64, t44436: f64, t446: f64, t5225: f64, t5299: f64, t5309: f64, t5424: f64, t56957: f64, t840: f64, t84795: f64, t84797: f64, t84823: f64, t84825: f64, t84856: f64, t84880: f64, t89: f64) -> (f64, f64) {
    let t91082 = t91032 + t91048 + t91065 + t91080;
    let t91104 = t89 * t193 * t295 * t91082 * t312 / 3.0_f64 + t44436 + 8.0_f64 / 27.0_f64 * t84795 + 8.0_f64 / 9.0_f64 * t84797 + 8.0_f64 / 3.0_f64 * t84823 + 4.0_f64 / 9.0_f64 * t84825 + 112.0_f64 / 81.0_f64 * t56957 - 2.0_f64 * t446 * t840 * t5424 * t5299 - 8.0_f64 / 3.0_f64 * t84856 - 16.0_f64 / 27.0_f64 * t84880 + 8.0_f64 * t446 * t2862 * t2843 * t5225 * t5309;
    (t91082, t91104)
}
