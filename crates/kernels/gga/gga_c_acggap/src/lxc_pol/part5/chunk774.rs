//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 774/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk774<F: Float>(t1922: F, t377: F, t407: F, t6482: F, t1539: F, t6465: F, t1160: F, t6461: F, t1411: F, t1629: F, t1533: F, t1907: F, t394: F, t1170: F, t151: F, t1530: F, t3833: F, t3835: F, t3842: F, t3843: F, t3846: F, t5305: F, t5318: F, t5327: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6529 = t377 * t1922;
    let t6532 = t6482 * t407;
    let t6535 = t6465 * t1539;
    let t6536 = t1160 * t6535;
    let t6538 = t6461 * t407;
    let t6541 = t1629 * t1411;
    let t6544 = t6465 * t407;
    let t6547 = t6461 * t1533;
    let t6551 = t394 * t1907;
    let t6552 = t6551 * t407;
    let t6555 = -0.65854491829355115987e0 * t3833 - 0.13170898365871023197e1 * t5305 + 0.13170898365871023197e1 * t6529 - 0.13170898365871023197e1 * t3835 + t5318 - 0.65854491829355115987e0 * t1170 * t6532 + 0.65854491829355115987e0 * t6536 - 0.13170898365871023197e1 * t1170 * t6538 - 0.13170898365871023197e1 * t1170 * t6541 - 0.65854491829355115987e0 * t1170 * t6544 + 0.26341796731742046394e1 * t1530 * t6547 + t5327 - t3842 + 0.65854491829355115987e0 * t3843 + t3846 - 0.65854491829355115987e0 * t151 * t6552;
    (t6532, t6535, t6538, t6541, t6544, t6547, t6551, t6552, t6555)
}
