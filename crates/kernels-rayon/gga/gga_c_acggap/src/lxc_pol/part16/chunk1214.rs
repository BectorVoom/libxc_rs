//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1214/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1214(t159: f64, t2155: f64, t2347: f64, t32030: f64, t32033: f64, t32036: f64, t32043: f64, t32048: f64, t32052: f64, t32054: f64, t33566: f64, t33747: f64, t33767: f64, t40595: f64, t40620: f64, t616: f64, t619: f64, t7912: f64, t8433: f64, t9003: f64, t9498: f64, t9774: f64) -> f64 {
    let t40791 = -0.26020884564615598386e1_f64 * t7912 * t9498 + 0.4336814094102599731e0_f64 * t7912 * t9774 + 0.8673628188205199462e0_f64 * t9003 * t8433 - t33747 + 0.8673628188205199462e0_f64 * t33566 * t2347 - 0.4336814094102599731e0_f64 * t616 * t619 * t159 * t40595 - 0.65854491829355115987e0_f64 * t32030 - 0.13170898365871023197e1_f64 * t32033 - 0.8673628188205199462e0_f64 * t32036 + 0.26020884564615598386e1_f64 * t32043 + t32048 + t32052 + 0.34694512752820797848e1_f64 * t33767 + t32054 + 0.4336814094102599731e0_f64 * t40620 * t2155;
    t40791
}
