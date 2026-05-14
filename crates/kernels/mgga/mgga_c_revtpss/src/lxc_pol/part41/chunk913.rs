//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 913/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk913<F: Float>(t225: F, t6888: F, t1903: F, t4076: F, t1437: F, t1883: F, t213: F, t4082: F, t4085: F, t4099: F, t4113: F, t4114: F, t546: F, t5738: F, t5742: F, t5761: F, t5765: F, t5767: F, t6844: F, t6862: F, t6874: F, t820: F) -> (F, F, F, F) {
    let t6889 = t6888 * t225;
    let t6895 = t1903 * t1903;
    let t6896 = t4076 * t6895;
    let t6918 = t4082 - t4085 + 0.10975748638225852664e-1 * t5738 - 0.10975748638225852664e-1 * t5761 + t4099 - 0.19514881078765566038e-1 * t5742 + 0.19514881078765566038e-1 * t5765 - t4113 + 0.13170898365871023197e1 * t820 * t4114 * t6862 - 0.13170898365871023197e1 * t820 * t5767 * t1883 - 0.65854491829355115987e0 * t820 * t1437 * t6844 - 0.65854491829355115987e0 * t820 * t1437 * t6874 + 0.65854491829355115987e0 * t213 * t546 * t6888;
    (t6889, t6895, t6896, t6918)
}
