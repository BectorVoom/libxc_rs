//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3624/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624<F: Float>(t16750: F, t1774: F, t1211: F, t1215: F, t12658: F, t1274: F, t1770: F, t17963: F, t17979: F, t17986: F, t17991: F, t18065: F, t18087: F, t1828: F, t21389: F, t21618: F, t21624: F, t3556: F, t3567: F, t3575: F, t3732: F, t3736: F, t3737: F, t45482: F, t5498: F, t6574: F, t6580: F, t6744: F, t68658: F) -> (F, F) {
    let t68669 = t1774 * t16750;
    let t68673 = -F::cast_from(0.26341796731742046394e1_f64) * t17986 * t21389 * t17991 - F::cast_from(0.13170898365871023197e1_f64) * t3732 * t21618 - F::cast_from(0.26341796731742046394e1_f64) * t18065 * t5498 + F::cast_from(0.13170898365871023197e1_f64) * t1770 * t17979 + F::cast_from(0.26341796731742046394e1_f64) * t1274 * t3737 * t1828 * t17963 - F::cast_from(0.26341796731742046394e1_f64) * t17986 * t3736 * t6744 * t3575 - F::cast_from(0.26341796731742046394e1_f64) * t68658 * t1215 - F::cast_from(0.13170898365871023197e1_f64) * t3556 * t21624 - F::cast_from(0.26341796731742046394e1_f64) * t18087 * t5498 + F::cast_from(0.13170898365871023197e1_f64) * t45482 * t6574 + F::cast_from(0.13170898365871023197e1_f64) * t12658 * t6580 + F::cast_from(0.26341796731742046394e1_f64) * t3567 * t1211 * t68669;
    (t68669, t68673)
}
