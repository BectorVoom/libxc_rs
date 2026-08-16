//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3552/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3552<F: Float>(t20112: F, t359: F, t19572: F, t3302: F, t3259: F, t6305: F, t1024: F, t1082: F, t11940: F, t12122: F, t12127: F, t15604: F, t15609: F, t15717: F, t16409: F, t16410: F, t1647: F, t16505: F, t16566: F, t19447: F, t19456: F, t19521: F, t19566: F, t19594: F, t19597: F, t3204: F, t3291: F, t3299: F, t3304: F, t3309: F, t3322: F, t342: F, t380: F, t43360: F, t43453: F, t43598: F, t4984: F, t4999: F, t5004: F, t55499: F, t6235: F, t64831: F, t66771: F, t67584: F, t999: F) -> (F, F) {
    let t67595 = t359 * t20112;
    let t67599 = t19572 * t3302;
    let t67618 = t3259 * t6305;
    let t67633 = F::cast_from(0.65854491829355115987e0_f64) * t342 * t380 * t67584 + F::cast_from(0.13170898365871023197e1_f64) * t19566 * t3309 + F::cast_from(0.65854491829355115987e0_f64) * t6235 * t3322 - F::cast_from(0.79025390195226139182e1_f64) * t11940 * t5004 * t15717 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t67595 * t999 - F::cast_from(0.26341796731742046394e1_f64) * t12122 * t67599 * t15609 + F::cast_from(0.26341796731742046394e1_f64) * t16566 * t55499 * t66771 + F::cast_from(0.13170898365871023197e1_f64) * t12127 * t67599 * t15604 - F::cast_from(0.79025390195226139182e1_f64) * t11940 * t3291 * t19456 - F::cast_from(0.52683593463484092788e1_f64) * t43360 * t19594 + F::cast_from(0.26341796731742046394e1_f64) * t43453 * t19597 + F::cast_from(0.52683593463484092788e1_f64) * t43598 * t19447 + F::cast_from(0.13170898365871023197e1_f64) * t3299 * t67618 * t3304 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t1082 * t64831 + F::cast_from(0.52683593463484092788e1_f64) * t16410 * t19521 + F::cast_from(0.52683593463484092788e1_f64) * t1647 * t16409 * t4984 - F::cast_from(0.26341796731742046394e1_f64) * t1647 * t16505 * t4999;
    (t67618, t67633)
}
