//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 774/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk774<F: Float>(t157: F, t2152: F, t8117: F, t119: F, t1222: F, t1265: F, t2143: F, t2146: F, t2222: F, t2245: F, t464: F, t616: F, t639: F, t7931: F, t7938: F, t8123: F, t8126: F, t8303: F, t8307: F, t8311: F, t8314: F, t8316: F, t8319: F, t8322: F, t8330: F, t8332: F, t8339: F, t8342: F, t8349: F) -> (F, F) {
    let t8351 = t2152 * t8117 * t157;
    let t8354 = F::cast_from(0.13170898365871023197e1_f64) * t8123 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8126 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t8303 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t8307 - F::cast_from(0.17347256376410398924e1_f64) * t8311 + F::cast_from(0.17347256376410398924e1_f64) * t8314 - F::cast_from(0.13170898365871023197e1_f64) * t8316 * t464 + F::cast_from(0.13170898365871023197e1_f64) * t8319 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t8322 - F::cast_from(0.65854491829355115987e0_f64) * t2222 * t1265 - F::cast_from(0.4336814094102599731e0_f64) * t7938 * t639 + t8330 - F::cast_from(0.13170898365871023197e1_f64) * t8332 + F::cast_from(0.13170898365871023197e1_f64) * t2222 * t1222 - t8339 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8342 - F::cast_from(0.8673628188205199462e0_f64) * t2143 * t2245 + t8349 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t8351;
    (t8351, t8354)
}
