//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 714/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk714<F: Float>(t1221: F, t609: F, t8004: F, t2122: F, t2147: F, t463: F, t1264: F, t119: F, t1222: F, t2127: F, t2146: F, t2149: F, t464: F, t7912: F, t7954: F, t7957: F, t7962: F, t7967: F, t7970: F, t7974: F, t7977: F, t7981: F, t7985: F, t7988: F, t7991: F, t7996: F, t8000: F, t8001: F) -> (F, F, F, F) {
    let t8006 = t8004 * t609 * t1221;
    let t8010 = t2147 * t2122 * t463;
    let t8013 = t609 * t1264;
    let t8014 = t2147 * t8013;
    let t8019 = 0.4336814094102599731e0 * t2146 * t7954 + 0.13170898365871023197e1 * t7957 + 0.13170898365871023197e1 * t2127 * t1222 + t7962 + 0.17347256376410398924e1 * t7967 + 0.65854491829355115987e0 * t119 * t7970 - 0.13170898365871023197e1 * t7974 - 0.13170898365871023197e1 * t7977 - 0.17347256376410398924e1 * t7981 + 0.17347256376410398924e1 * t7985 - 0.17347256376410398924e1 * t7988 + 0.17347256376410398924e1 * t7991 + t7996 - t8000 - 0.13170898365871023197e1 * t8001 * t464 - 0.26020884564615598386e1 * t2146 * t8006 + 0.17347256376410398924e1 * t2146 * t8010 + 0.8673628188205199462e0 * t2146 * t8014 + 0.17347256376410398924e1 * t7912 * t2149;
    (t8006, t8010, t8014, t8019)
}
