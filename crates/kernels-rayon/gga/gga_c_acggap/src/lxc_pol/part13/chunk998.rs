//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 998/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk998(t2131: f64, t2147: f64, t2341: f64, t847: f64, t2331: f64, t862: f64, t865: f64, t32030: f64, t32033: f64, t32036: f64, t32039: f64, t32043: f64, t32048: f64, t32052: f64, t32054: f64, t32057: f64, t32061: f64, t32064: f64, t32069: f64, t557: f64, t7917: f64, t9003: f64) -> f64 {
    let t33767 = t2131 * t2147 * t2341 * t847;
    let t33771 = t862 * t2331 * t865;
    let t33775 = -0.13170898365871023197e1_f64 * t32030 - 0.26341796731742046394e1_f64 * t32033 + 0.8673628188205199462e0_f64 * t9003 * t7917 - 0.17347256376410398924e1_f64 * t32036 - 0.8673628188205199462e0_f64 * t32039 + 0.52041769129231196772e1_f64 * t32043 - 0.65854491829355115987e0_f64 * t32069 * t557 + t32048 + t32052 + 0.17347256376410398924e1_f64 * t33767 + t32054 - 0.52041769129231196772e1_f64 * t32057 + 0.13170898365871023197e1_f64 * t33771 + 0.10408353825846239354e2_f64 * t32061 - 0.8673628188205199462e0_f64 * t32064;
    t33775
}
