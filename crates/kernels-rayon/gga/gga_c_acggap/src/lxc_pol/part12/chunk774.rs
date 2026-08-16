//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 774/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk774(t157: f64, t2152: f64, t8117: f64, t119: f64, t1222: f64, t1265: f64, t2143: f64, t2146: f64, t2222: f64, t2245: f64, t464: f64, t616: f64, t639: f64, t7931: f64, t7938: f64, t8123: f64, t8126: f64, t8303: f64, t8307: f64, t8311: f64, t8314: f64, t8316: f64, t8319: f64, t8322: f64, t8330: f64, t8332: f64, t8339: f64, t8342: f64, t8349: f64) -> (f64, f64) {
    let t8351 = t2152 * t8117 * t157;
    let t8354 = 0.13170898365871023197e1_f64 * t8123 + 0.8673628188205199462e0_f64 * t2146 * t8126 - 0.4336814094102599731e0_f64 * t616 * t8303 - 0.17347256376410398924e1_f64 * t7931 * t8307 - 0.17347256376410398924e1_f64 * t8311 + 0.17347256376410398924e1_f64 * t8314 - 0.13170898365871023197e1_f64 * t8316 * t464 + 0.13170898365871023197e1_f64 * t8319 + 0.65854491829355115987e0_f64 * t119 * t8322 - 0.65854491829355115987e0_f64 * t2222 * t1265 - 0.4336814094102599731e0_f64 * t7938 * t639 + t8330 - 0.13170898365871023197e1_f64 * t8332 + 0.13170898365871023197e1_f64 * t2222 * t1222 - t8339 + 0.8673628188205199462e0_f64 * t2146 * t8342 - 0.8673628188205199462e0_f64 * t2143 * t2245 + t8349 + 0.4336814094102599731e0_f64 * t2146 * t8351;
    (t8351, t8354)
}
