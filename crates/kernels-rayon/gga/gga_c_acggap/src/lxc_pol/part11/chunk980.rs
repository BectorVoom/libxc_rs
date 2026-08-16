//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 980/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk980(t1265: f64, t2122: f64, t2146: f64, t29973: f64, t29977: f64, t29982: f64, t29986: f64, t29989: f64, t33414: f64, t33416: f64, t33431: f64, t33435: f64, t33437: f64, t463: f64, t7912: f64, t8004: f64, t8392: f64, t8400: f64, t8411: f64, t8791: f64, t9010: f64, t939: f64) -> f64 {
    let t33441 = t33414 - t33416 - 0.17347256376410398924e1_f64 * t8400 * t939 * t2122 * t8791 - t29973 - 0.52041769129231196772e1_f64 * t7912 * t8411 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t8392 * t463 - 0.52041769129231196772e1_f64 * t29977 + t33431 - 0.65854491829355115987e0_f64 * t9010 * t1265 + t33435 - t33437 - 0.13877805101128319139e2_f64 * t29982 + 0.8673628188205199462e0_f64 * t29986 - 0.13170898365871023197e1_f64 * t29989;
    t33441
}
