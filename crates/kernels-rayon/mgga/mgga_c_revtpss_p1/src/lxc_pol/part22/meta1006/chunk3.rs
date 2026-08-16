//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3441/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441(t1678: f64, t4743: f64, t11120: f64, t1651: f64, t1079: f64, t1097: f64, t11210: f64, t15886: f64, t16287: f64, t16321: f64, t16322: f64, t16327: f64, t16591: f64, t16592: f64, t16603: f64, t16604: f64, t1680: f64, t1696: f64, t19429: f64, t3058: f64, t3059: f64, t4752: f64, t4778: f64, t4935: f64, t53027: f64, t55416: f64, t6392: f64, t6393: f64, t995: f64) -> f64 {
    let t64605 = t4743 * t1678;
    let t64614 = t11120 * t1651;
    let t64626 = -0.13170898365871023197e1_f64 * t55416 * t1696 - 0.13170898365871023197e1_f64 * t3058 * t1079 * t6392 * t3059 - 0.52683593463484092788e1_f64 * t16603 * t16604 * t16327 - 0.65854491829355115987e0_f64 * t11210 * t6393 - 0.26341796731742046394e1_f64 * t64605 * t1097 - 0.52683593463484092788e1_f64 * t53027 * t19429 + 0.13170898365871023197e1_f64 * t995 * t1079 * t1651 * t16591 + 0.79025390195226139182e1_f64 * t16603 * t64614 * t16321 - 0.13170898365871023197e1_f64 * t4778 * t16287 - 0.13170898365871023197e1_f64 * t4935 * t16592 + 0.13170898365871023197e1_f64 * t15886 * t1680 - 0.79025390195226139182e1_f64 * t4752 * t16322;
    t64626
}
