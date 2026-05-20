//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3441/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3441<F: Float>(t1678: F, t4743: F, t11120: F, t1651: F, t1079: F, t1097: F, t11210: F, t15886: F, t16287: F, t16321: F, t16322: F, t16327: F, t16591: F, t16592: F, t16603: F, t16604: F, t1680: F, t1696: F, t19429: F, t3058: F, t3059: F, t4752: F, t4778: F, t4935: F, t53027: F, t55416: F, t6392: F, t6393: F, t995: F) -> F {
    let t64605 = t4743 * t1678;
    let t64614 = t11120 * t1651;
    let t64626 = -F::cast_from(0.13170898365871023197e1_f64) * t55416 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t3058 * t1079 * t6392 * t3059 - F::cast_from(0.52683593463484092788e1_f64) * t16603 * t16604 * t16327 - F::cast_from(0.65854491829355115987e0_f64) * t11210 * t6393 - F::cast_from(0.26341796731742046394e1_f64) * t64605 * t1097 - F::cast_from(0.52683593463484092788e1_f64) * t53027 * t19429 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t1079 * t1651 * t16591 + F::cast_from(0.79025390195226139182e1_f64) * t16603 * t64614 * t16321 - F::cast_from(0.13170898365871023197e1_f64) * t4778 * t16287 - F::cast_from(0.13170898365871023197e1_f64) * t4935 * t16592 + F::cast_from(0.13170898365871023197e1_f64) * t15886 * t1680 - F::cast_from(0.79025390195226139182e1_f64) * t4752 * t16322;
    t64626
}
