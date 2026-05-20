//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3463/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3463<F: Float>(t12166: F, t1678: F, t342: F, t12077: F, t11782: F, t12050: F, t15655: F, t16390: F, t16410: F, t16446: F, t16468: F, t16479: F, t16499: F, t16502: F, t16506: F, t16544: F, t16555: F, t16562: F, t16566: F, t19399: F, t19443: F, t19450: F, t19457: F, t19573: F, t19576: F, t3133: F, t3204: F, t3223: F, t3291: F, t3316: F, t357: F, t42261: F, t4857: F, t4930: F, t4967: F, t4999: F, t53877: F, t6368: F) -> F {
    let t65216 = t342 * t12166 * t1678;
    let t65220 = t342 * t12077 * t1678;
    let t65239 = F::cast_from(0.26341796731742046394e1_f64) * t16410 * t19573 - F::cast_from(0.13170898365871023197e1_f64) * t16506 * t19576 - F::cast_from(0.26341796731742046394e1_f64) * t15655 * t4967 - F::cast_from(0.13170898365871023197e1_f64) * t11782 * t6368 - F::cast_from(0.13170898365871023197e1_f64) * t4857 * t16446 - F::cast_from(0.13170898365871023197e1_f64) * t4857 * t16479 - F::cast_from(0.79025390195226139182e1_f64) * t53877 * t16499 - F::cast_from(0.26341796731742046394e1_f64) * t342 * t3316 * t4930 * t4999 + F::cast_from(0.79025390195226139182e1_f64) * t65216 * t16555 - F::cast_from(0.79025390195226139182e1_f64) * t65220 * t16562 + F::cast_from(0.65854491829355115987e0_f64) * t16566 * t19450 * t12050 * t3133 * t357 + F::cast_from(0.52683593463484092788e1_f64) * t3204 * t3291 * t19399 - F::cast_from(0.79025390195226139182e1_f64) * t42261 * t19457 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t19443 - F::cast_from(0.26341796731742046394e1_f64) * t16502 * t16468 - F::cast_from(0.26341796731742046394e1_f64) * t16544 * t16390;
    t65239
}
