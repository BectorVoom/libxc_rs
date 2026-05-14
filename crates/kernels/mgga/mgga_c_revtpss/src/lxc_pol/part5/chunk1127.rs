//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1127/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1127<F: Float>(t19450: F, t19548: F, t1093: F, t11788: F, t12160: F, t15655: F, t16502: F, t16544: F, t16552: F, t1685: F, t19509: F, t19512: F, t19515: F, t19521: F, t19526: F, t19534: F, t19539: F, t3204: F, t3223: F, t3299: F, t3317: F, t4857: F, t4964: F, t4967: F, t4977: F, t4981: F, t4984: F, t6235: F, t6362: F, t6371: F, t6386: F) -> (F,) {
    let t19549 = t19450 * t19548;
    let t19554 = 0.26341796731742046394e1 * t3204 * t19509 + 0.13170898365871023197e1 * t3204 * t19512 + 0.26341796731742046394e1 * t3204 * t19515 + 0.13170898365871023197e1 * t11788 * t6362 + 0.26341796731742046394e1 * t4981 * t19521 - 0.13170898365871023197e1 * t16544 * t4964 + 0.26341796731742046394e1 * t19526 * t4984 - 0.13170898365871023197e1 * t4857 * t4967 - 0.13170898365871023197e1 * t15655 * t1685 - 0.65854491829355115987e0 * t3317 * t19534 - 0.65854491829355115987e0 * t12160 * t6386 + 0.13170898365871023197e1 * t3299 * t19539 - 0.65854491829355115987e0 * t3223 * t6371 - 0.13170898365871023197e1 * t16502 * t4964 - 0.13170898365871023197e1 * t16502 * t4977 + 0.39512695097613069591e1 * t16552 * t19549 + 0.65854491829355115987e0 * t6235 * t1093;
    (t19554,)
}
