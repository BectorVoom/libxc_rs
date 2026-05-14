//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1155/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1155<F: Float>(t20112: F, t380: F, t1043: F, t1089: F, t6343: F, t1668: F, t4930: F, t16449: F, t1651: F, t4772: F, t5004: F, t20089: F, t19829: F, t19836: F, t1024: F, t1087: F, t12146: F, t12149: F, t12154: F, t15670: F, t19608: F, t19612: F, t19617: F, t19856: F, t3204: F, t3278: F, t3287: F, t342: F, t381: F, t4961: F, t4999: F, t6365: F, t6379: F, t6389: F, t989: F) -> (F,) {
    let t20113 = t380 * t20112;
    let t20119 = t6343 * t1043 * t1089;
    let t20123 = t4930 * t1668 * t1089;
    let t20128 = t16449 * t1651;
    let t20133 = t5004 * t4772;
    let t20136 = t20089 * t1089;
    let t20139 = t19829 * t1089;
    let t20146 = t19836 * t1089;
    let t20149 = -0.13170898365871023197e1 * t19608 * t4999 - 0.65854491829355115987e0 * t3287 * t19612 + 0.65854491829355115987e0 * t989 * t6389 + 0.13170898365871023197e1 * t3204 * t19617 + 0.65854491829355115987e0 * t342 * t20113 + 0.65854491829355115987e0 * t19856 * t381 + 0.65854491829355115987e0 * t1087 * t20119 + 0.13170898365871023197e1 * t1087 * t20123 + 0.13170898365871023197e1 * t3278 * t6379 - 0.13170898365871023197e1 * t1024 * t20128 + 0.26341796731742046394e1 * t15670 * t4961 - 0.13170898365871023197e1 * t1024 * t20133 - 0.13170898365871023197e1 * t3287 * t20136 + 0.13170898365871023197e1 * t12149 * t20139 - 0.13170898365871023197e1 * t12146 * t6365 - 0.13170898365871023197e1 * t12154 * t6365 - 0.13170898365871023197e1 * t3287 * t20146;
    (t20149,)
}
