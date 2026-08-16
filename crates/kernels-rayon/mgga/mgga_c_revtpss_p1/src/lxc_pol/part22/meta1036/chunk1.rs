//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3624/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3624(t16750: f64, t1774: f64, t1211: f64, t1215: f64, t12658: f64, t1274: f64, t1770: f64, t17963: f64, t17979: f64, t17986: f64, t17991: f64, t18065: f64, t18087: f64, t1828: f64, t21389: f64, t21618: f64, t21624: f64, t3556: f64, t3567: f64, t3575: f64, t3732: f64, t3736: f64, t3737: f64, t45482: f64, t5498: f64, t6574: f64, t6580: f64, t6744: f64, t68658: f64) -> (f64, f64) {
    let t68669 = t1774 * t16750;
    let t68673 = -0.26341796731742046394e1_f64 * t17986 * t21389 * t17991 - 0.13170898365871023197e1_f64 * t3732 * t21618 - 0.26341796731742046394e1_f64 * t18065 * t5498 + 0.13170898365871023197e1_f64 * t1770 * t17979 + 0.26341796731742046394e1_f64 * t1274 * t3737 * t1828 * t17963 - 0.26341796731742046394e1_f64 * t17986 * t3736 * t6744 * t3575 - 0.26341796731742046394e1_f64 * t68658 * t1215 - 0.13170898365871023197e1_f64 * t3556 * t21624 - 0.26341796731742046394e1_f64 * t18087 * t5498 + 0.13170898365871023197e1_f64 * t45482 * t6574 + 0.13170898365871023197e1_f64 * t12658 * t6580 + 0.26341796731742046394e1_f64 * t3567 * t1211 * t68669;
    (t68669, t68673)
}
