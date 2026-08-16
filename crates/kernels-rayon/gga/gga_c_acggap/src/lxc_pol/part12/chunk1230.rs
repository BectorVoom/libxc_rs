//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1230/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1230(t2146: f64, t33080: f64, t33085: f64, t33088: f64, t33090: f64, t33093: f64, t33097: f64, t33100: f64, t33104: f64, t33107: f64, t33561: f64, t38153: f64, t38157: f64, t38165: f64, t38176: f64, t463: f64, t7931: f64, t8004: f64, t8306: f64, t9417: f64) -> f64 {
    let t38178 = t38153 + t38157 - 0.8673628188205199462e0_f64 * t7931 * t8306 * t33561 - 0.26341796731742046395e1_f64 * t33080 + t38165 + 0.8673628188205199462e0_f64 * t33085 - 0.8673628188205199462e0_f64 * t33088 - 0.13170898365871023197e1_f64 * t33090 - 0.13170898365871023197e1_f64 * t33093 - 0.17347256376410398924e1_f64 * t33097 - t33100 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t9417 * t463 + t38176 + 0.17347256376410398924e1_f64 * t33104 - t33107;
    t38178
}
