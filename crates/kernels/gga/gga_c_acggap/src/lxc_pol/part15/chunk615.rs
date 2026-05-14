//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 615/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk615<F: Float>(t322: F, t6454: F, t381: F, t524: F, t545: F, t1539: F, t1160: F, t180: F, t1814: F, t3457: F, t3073: F, t1629: F, t6263: F, t5853: F, t1533: F, t1530: F, t3031: F, t3040: F, t3047: F, t4152: F, t4164: F, t4170: F, t4182: F, t4185: F, t4188: F, t4198: F) -> (F, F, F) {
    let t6455 = t6454 * t322;
    let t6456 = t381 * t6455;
    let t6461 = t545 * t524;
    let t6462 = t6461 * t1539;
    let t6463 = t1160 * t6462;
    let t6465 = t180 * t1814;
    let t6466 = t6465 * t3457;
    let t6467 = t3073 * t6466;
    let t6469 = t1629 * t6263;
    let t6472 = t6465 * t5853;
    let t6475 = t6465 * t1533;
    let t6479 = -0.65854491829355115987e0 * t6456 + t4152 - 0.65854491829355115987e0 * t3031 + 0.13170898365871023197e1 * t4164 - 0.26341796731742046394e1 * t4170 + 0.13170898365871023197e1 * t6463 - 0.13170898365871023197e1 * t6467 - t3040 + t3047 + 0.26341796731742046394e1 * t1530 * t6469 - 0.39512695097613069591e1 * t4198 * t6472 + 0.39512695097613069591e1 * t1530 * t6475 + t4182 - 0.26341796731742046394e1 * t4185 - t4188;
    (t6461, t6465, t6479)
}
