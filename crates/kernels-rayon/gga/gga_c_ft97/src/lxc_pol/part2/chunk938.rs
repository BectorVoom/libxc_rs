//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 938/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk938(t14555: f64, t898: f64, t900: f64, t10835: f64, t10838: f64, t10839: f64, t10841: f64, t10843: f64, t10921: f64, t10923: f64, t10925: f64, t10927: f64, t12143: f64, t14484: f64, t14488: f64, t14491: f64, t14497: f64, t14501: f64, t14503: f64, t14507: f64, t14516: f64, t14520: f64, t14524: f64, t2265: f64, t631: f64) -> f64 {
    let t14557 = t898 * t900 * t14555;
    let t14560 = t2265 * t14484 / 18.0_f64 + 2.0_f64 / 27.0_f64 * t2265 * t14488 - 2.0_f64 / 9.0_f64 * t12143 * t14491 + t10838 + 10.0_f64 / 27.0_f64 * t10921 - t10923 / 9.0_f64 - t10925 / 27.0_f64 - t2265 * t14497 / 3.0_f64 + t2265 * t14501 + t2265 * t14503 + 2.0_f64 / 3.0_f64 * t2265 * t14507 + 4.0_f64 / 9.0_f64 * t10841 + 2.0_f64 / 9.0_f64 * t10843 - t10835 / 3.0_f64 + 10.0_f64 / 9.0_f64 * t10839 - 2.0_f64 / 9.0_f64 * t2265 * t14516 + 4.0_f64 / 3.0_f64 * t2265 * t14520 + 2.0_f64 * t2265 * t14524 + t10927 + t631 * t14557 / 2.0_f64;
    t14560
}
