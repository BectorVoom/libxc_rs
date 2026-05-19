//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1307/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1307<F: Float>(t15957: F, t4976: F, t1024: F, t1087: F, t11782: F, t11788: F, t12122: F, t12127: F, t12149: F, t16427: F, t16433: F, t16436: F, t16440: F, t16443: F, t16446: F, t16450: F, t16458: F, t16461: F, t16465: F, t1685: F, t1692: F, t3043: F, t3223: F, t3278: F, t3287: F, t3299: F, t3313: F, t4954: F, t4961: F, t4981: F, t4988: F, t5005: F) -> F {
    let t16468 = t15957 * t4976;
    let t16475 = F::cast_from(0.65854491829355115987e0_f64) * t4954 * t3313 + F::cast_from(0.13170898365871023197e1_f64) * t3299 * t16427 + F::cast_from(0.26341796731742046394e1_f64) * t11788 * t4961 - F::cast_from(0.26341796731742046394e1_f64) * t12122 * t16433 + F::cast_from(0.13170898365871023197e1_f64) * t12127 * t16436 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t16440 + F::cast_from(0.26341796731742046394e1_f64) * t4981 * t16443 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t16446 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t16450 - F::cast_from(0.65854491829355115987e0_f64) * t11782 * t1685 + F::cast_from(0.65854491829355115987e0_f64) * t3043 * t1692 + F::cast_from(0.26341796731742046394e1_f64) * t12149 * t16458 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t16461 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t16465 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t16468 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t4988 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t5005;
    t16475
}
