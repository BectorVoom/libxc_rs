//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3455/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3455<F: Float>(t64945: F, t64959: F, t64973: F, t64987: F, t1024: F, t1043: F, t1082: F, t1083: F, t1089: F, t11788: F, t12149: F, t16414: F, t16436: F, t16458: F, t1651: F, t1685: F, t1689: F, t19380: F, t19414: F, t19479: F, t19515: F, t3204: F, t3278: F, t3291: F, t3298: F, t342: F, t4743: F, t4930: F, t4954: F, t4984: F, t5012: F, t53865: F, t55649: F, t55747: F, t55868: F, t55991: F, t64907: F, t64912: F, t64916: F) -> (F, F) {
    let t64989 = t64945 + t64959 + t64973 + t64987;
    let t64997 = F::cast_from(0.26341796731742046394e1_f64) * t4743 * t5012 + F::cast_from(0.26341796731742046394e1_f64) * t4954 * t16414 + F::cast_from(0.52683593463484092788e1_f64) * t342 * t3298 * t4930 * t4984 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t19479 - F::cast_from(0.13170898365871023197e1_f64) * t64907 * t1083 + F::cast_from(0.13170898365871023197e1_f64) * t55868 * t1689 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t1082 * t64912 + F::cast_from(0.13170898365871023197e1_f64) * t12149 * t64916 * t1089 + F::cast_from(0.26341796731742046394e1_f64) * t55991 * t16436 + F::cast_from(0.26341796731742046394e1_f64) * t12149 * t19414 * t1043 * t1089 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t3291 * t19380 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t55649 * t1651 - F::cast_from(0.13170898365871023197e1_f64) * t53865 * t1685 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1082 * t64989 + F::cast_from(0.52683593463484092788e1_f64) * t11788 * t19515 + F::cast_from(0.52683593463484092788e1_f64) * t55747 * t16458;
    (t64989, t64997)
}
