//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1213/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1213(t33799: f64, t8407: f64, t1410: f64, t157: f64, t1658: f64, t2143: f64, t2146: f64, t2152: f64, t2331: f64, t2341: f64, t32001: f64, t33726: f64, t33818: f64, t40735: f64, t40738: f64, t40740: f64, t40746: f64, t40749: f64, t557: f64, t7912: f64, t7931: f64, t7932: f64, t8004: f64, t9503: f64, t9769: f64, t9801: f64) -> f64 {
    let t40764 = t33799 * t8407;
    let t40771 = 0.8673628188205199462e0_f64 * t40735 - 0.8673628188205199462e0_f64 * t40738 - 0.8673628188205199462e0_f64 * t7931 * t7932 * t40740 - 0.4336814094102599731e0_f64 * t2143 * t9769 - 0.13170898365871023197e1_f64 * t40746 - 0.17347256376410398924e1_f64 * t7931 * t7932 * t40749 + 0.65854491829355115987e0_f64 * t32001 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t2341 * t1658 - 0.13170898365871023197e1_f64 * t33818 * t557 + 0.8673628188205199462e0_f64 * t7912 * t9801 + 0.4336814094102599731e0_f64 * t7912 * t9503 - 0.17347256376410398924e1_f64 * t40764 - t33726 + 0.8673628188205199462e0_f64 * t2146 * t2152 * t2331 * t1410 * t157;
    t40771
}
