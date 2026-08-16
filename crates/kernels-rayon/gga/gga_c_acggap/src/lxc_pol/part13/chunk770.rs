//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 770/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk770(t1221: f64, t609: f64, t8004: f64, t2122: f64, t2147: f64, t463: f64, t1264: f64, t119: f64, t1222: f64, t2127: f64, t2146: f64, t2149: f64, t464: f64, t7912: f64, t7954: f64, t7957: f64, t7962: f64, t7967: f64, t7970: f64, t7974: f64, t7977: f64, t7981: f64, t7985: f64, t7988: f64, t7991: f64, t7996: f64, t8000: f64, t8001: f64) -> (f64, f64, f64, f64) {
    let t8006 = t8004 * t609 * t1221;
    let t8010 = t2147 * t2122 * t463;
    let t8013 = t609 * t1264;
    let t8014 = t2147 * t8013;
    let t8019 = 0.4336814094102599731e0_f64 * t2146 * t7954 + 0.13170898365871023197e1_f64 * t7957 + 0.13170898365871023197e1_f64 * t2127 * t1222 + t7962 + 0.17347256376410398924e1_f64 * t7967 + 0.65854491829355115987e0_f64 * t119 * t7970 - 0.13170898365871023197e1_f64 * t7974 - 0.13170898365871023197e1_f64 * t7977 - 0.17347256376410398924e1_f64 * t7981 + 0.17347256376410398924e1_f64 * t7985 - 0.17347256376410398924e1_f64 * t7988 + 0.17347256376410398924e1_f64 * t7991 + t7996 - t8000 - 0.13170898365871023197e1_f64 * t8001 * t464 - 0.26020884564615598386e1_f64 * t2146 * t8006 + 0.17347256376410398924e1_f64 * t2146 * t8010 + 0.8673628188205199462e0_f64 * t2146 * t8014 + 0.17347256376410398924e1_f64 * t7912 * t2149;
    (t8006, t8010, t8014, t8019)
}
