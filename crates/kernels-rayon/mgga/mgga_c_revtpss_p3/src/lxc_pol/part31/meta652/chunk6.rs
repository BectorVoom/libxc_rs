//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2170/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2170(t100708: f64, t1089: f64, t1646: f64, t1647: f64, t1652: f64, t19396: f64, t1978: f64, t19856: f64, t25473: f64, t25634: f64, t27437: f64, t27543: f64, t27545: f64, t27604: f64, t27639: f64, t27643: f64, t27647: f64, t27665: f64, t27668: f64, t27670: f64, t27699: f64, t29752: f64, t29807: f64, t4743: f64, t4866: f64, t5016: f64, t6351: f64, t7102: f64, t7144: f64, t7145: f64, t7151: f64, t7167: f64, t7812: f64, t7825: f64, t999: f64, t99909: f64, t99915: f64) -> f64 {
    let t107691 = -0.26020884564615598386e1_f64 * t25473 * t29752 + 0.13170898365871023197e1_f64 * t1647 * t27545 + 0.65854491829355115987e0_f64 * t19856 * t1978 + 0.17347256376410398924e1_f64 * t99909 * t27647 + 0.17347256376410398924e1_f64 * t99915 * t27437 + 0.8673628188205199462e0_f64 * t7151 * t7145 * t29807 * t999 - 0.17347256376410398924e1_f64 * t7144 * t7145 * t27543 * t1646 + 0.13170898365871023197e1_f64 * t25634 * t6351 + 0.13170898365871023197e1_f64 * t7102 * t19396 + 0.17347256376410398924e1_f64 * t99909 * t27665 - 0.17347256376410398924e1_f64 * t7825 * t27668 * t27670 + 0.8673628188205199462e0_f64 * t7825 * t27639 * t27643 - 0.13170898365871023197e1_f64 * t27699 * t5016 + 0.13170898365871023197e1_f64 * t4743 * t7812 - 0.13170898365871023197e1_f64 * t100708 * t1652 - 0.8673628188205199462e0_f64 * t7167 * t27604 * t4866 * t1089;
    t107691
}
