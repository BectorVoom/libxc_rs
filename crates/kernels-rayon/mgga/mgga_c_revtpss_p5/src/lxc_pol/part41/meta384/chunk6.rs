//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1274/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1274(t19501: f64, t19579: f64, t1089: f64, t1678: f64, t4866: f64, t3153: f64, t6271: f64, t4983: f64, t4998: f64, t3298: f64, t342: f64, t1024: f64, t1087: f64, t1090: f64, t12116: f64, t12122: f64, t12127: f64, t16381: f64, t1647: f64, t1689: f64, t1692: f64, t19557: f64, t19566: f64, t19569: f64, t19573: f64, t19576: f64, t3278: f64, t4743: f64, t4857: f64, t4954: f64, t4970: f64, t4981: f64, t4984: f64, t4996: f64, t4999: f64, t5009: f64, t5012: f64, t6375: f64, t6383: f64) -> f64 {
    let t19580 = t19501 * t19579;
    let t19584 = t1678 * t4866 * t1089;
    let t19593 = t6271 * t3153;
    let t19594 = t19593 * t4983;
    let t19597 = t19593 * t4998;
    let t19602 = t3298 * t1678;
    let t19603 = t342 * t19602;
    let t19606 = -0.65854491829355115987e0_f64 * t1024 * t19557 - 0.13170898365871023197e1_f64 * t4857 * t4970 + 0.13170898365871023197e1_f64 * t12116 * t6375 + 0.13170898365871023197e1_f64 * t1647 * t5012 + 0.65854491829355115987e0_f64 * t19566 * t1090 - 0.13170898365871023197e1_f64 * t19569 * t4999 + 0.13170898365871023197e1_f64 * t4981 * t19573 - 0.65854491829355115987e0_f64 * t4996 * t19576 + 0.65854491829355115987e0_f64 * t12127 * t19580 + 0.13170898365871023197e1_f64 * t1087 * t19584 + 0.65854491829355115987e0_f64 * t3278 * t6383 + 0.13170898365871023197e1_f64 * t16381 * t1689 + 0.13170898365871023197e1_f64 * t4743 * t1692 - 0.26341796731742046394e1_f64 * t12122 * t19594 + 0.13170898365871023197e1_f64 * t12127 * t19597 + 0.13170898365871023197e1_f64 * t4954 * t5009 + 0.26341796731742046394e1_f64 * t19603 * t4984;
    t19606
}
