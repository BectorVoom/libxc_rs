//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 771/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk771<F: Float>(t1533: F, t6482: F, t1539: F, t1160: F, t1530: F, t3057: F, t3059: F, t3067: F, t3078: F, t3091: F, t3104: F, t4191: F, t4192: F, t4213: F, t4215: F, t4228: F, t4230: F, t4231: F, t4234: F) -> (F, F, F) {
    let t6483 = t6482 * t1533;
    let t6489 = t6482 * t1539;
    let t6490 = t1160 * t6489;
    let t6493 = t4191 + t3057 + 0.26341796731742046394e1 * t4192 + 0.13170898365871023197e1 * t3059 + t4213 - t4215 + 0.13170898365871023197e1 * t1530 * t6483 + 0.65854491829355115987e0 * t3067 + 0.13170898365871023197e1 * t3078 - 0.13170898365871023197e1 * t3091 - t4228 + 0.65854491829355115987e0 * t6490 - t4230 - 0.13170898365871023197e1 * t4231 - t4234 - t3104;
    (t6483, t6489, t6493)
}
