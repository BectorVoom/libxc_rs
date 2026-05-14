//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 717/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk717<F: Float>(t464: F, t8331: F, t633: F, t864: F, t2132: F, t7885: F, t157: F, t2217: F, t406: F, t2152: F, t862: F, t865: F, t8117: F, t119: F, t1222: F, t1265: F, t2143: F, t2146: F, t2222: F, t2245: F, t616: F, t639: F, t7931: F, t7938: F, t8123: F, t8126: F, t8303: F, t8307: F, t8311: F, t8314: F, t8316: F, t8319: F, t8322: F, t8330: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8332 = t8331 * t464;
    let t8336 = t633 * t864;
    let t8337 = t2132 * t8336;
    let t8339 = 0.26020884564615598386e1 * t7885 * t8337;
    let t8341 = t2217 * t406 * t157;
    let t8342 = t2152 * t8341;
    let t8347 = t862 * t633;
    let t8349 = 0.13170898365871023197e1 * t8347 * t865;
    let t8351 = t2152 * t8117 * t157;
    let t8354 = 0.13170898365871023197e1 * t8123 + 0.8673628188205199462e0 * t2146 * t8126 - 0.4336814094102599731e0 * t616 * t8303 - 0.17347256376410398924e1 * t7931 * t8307 - 0.17347256376410398924e1 * t8311 + 0.17347256376410398924e1 * t8314 - 0.13170898365871023197e1 * t8316 * t464 + 0.13170898365871023197e1 * t8319 + 0.65854491829355115987e0 * t119 * t8322 - 0.65854491829355115987e0 * t2222 * t1265 - 0.4336814094102599731e0 * t7938 * t639 + t8330 - 0.13170898365871023197e1 * t8332 + 0.13170898365871023197e1 * t2222 * t1222 - t8339 + 0.8673628188205199462e0 * t2146 * t8342 - 0.8673628188205199462e0 * t2143 * t2245 + t8349 + 0.4336814094102599731e0 * t2146 * t8351;
    (t8332, t8336, t8337, t8339, t8342, t8347, t8349, t8351, t8354)
}
