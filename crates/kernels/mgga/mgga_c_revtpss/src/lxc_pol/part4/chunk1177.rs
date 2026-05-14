//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1177/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1177<F: Float>(t16449: F, t999: F, t1043: F, t1089: F, t4757: F, t3291: F, t4772: F, t1678: F, t3133: F, t15957: F, t4976: F, t1024: F, t1087: F, t11782: F, t11788: F, t12122: F, t12127: F, t12149: F, t16427: F, t16433: F, t16436: F, t16440: F, t16443: F, t16446: F, t1685: F, t1692: F, t3043: F, t3223: F, t3278: F, t3287: F, t3299: F, t3313: F, t4954: F, t4961: F, t4981: F, t4988: F, t5005: F) -> (F,) {
    let t16450 = t16449 * t999;
    let t16458 = t4757 * t1043 * t1089;
    let t16461 = t3291 * t4772;
    let t16465 = t1678 * t3133 * t1089;
    let t16468 = t15957 * t4976;
    let t16475 = 0.65854491829355115987e0 * t4954 * t3313 + 0.13170898365871023197e1 * t3299 * t16427 + 0.26341796731742046394e1 * t11788 * t4961 - 0.26341796731742046394e1 * t12122 * t16433 + 0.13170898365871023197e1 * t12127 * t16436 + 0.65854491829355115987e0 * t1087 * t16440 + 0.26341796731742046394e1 * t4981 * t16443 - 0.65854491829355115987e0 * t1024 * t16446 - 0.13170898365871023197e1 * t1024 * t16450 - 0.65854491829355115987e0 * t11782 * t1685 + 0.65854491829355115987e0 * t3043 * t1692 + 0.26341796731742046394e1 * t12149 * t16458 - 0.13170898365871023197e1 * t1024 * t16461 + 0.65854491829355115987e0 * t1087 * t16465 - 0.13170898365871023197e1 * t3287 * t16468 + 0.13170898365871023197e1 * t3278 * t4988 - 0.13170898365871023197e1 * t3223 * t5005;
    (t16475,)
}
