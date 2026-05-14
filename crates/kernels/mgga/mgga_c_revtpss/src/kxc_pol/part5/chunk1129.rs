//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1129/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1129<F: Float>(t3153: F, t6271: F, t4983: F, t4998: F, t1678: F, t3298: F, t342: F, t1024: F, t1087: F, t1090: F, t12116: F, t12122: F, t12127: F, t16381: F, t1647: F, t1689: F, t1692: F, t19557: F, t19566: F, t19569: F, t19573: F, t19576: F, t19580: F, t19584: F, t3278: F, t4743: F, t4857: F, t4954: F, t4970: F, t4981: F, t4984: F, t4996: F, t4999: F, t5009: F, t5012: F, t6375: F, t6383: F) -> (F,) {
    let t19593 = t6271 * t3153;
    let t19594 = t19593 * t4983;
    let t19597 = t19593 * t4998;
    let t19602 = t3298 * t1678;
    let t19603 = t342 * t19602;
    let t19606 = -0.65854491829355115987e0 * t1024 * t19557 - 0.13170898365871023197e1 * t4857 * t4970 + 0.13170898365871023197e1 * t12116 * t6375 + 0.13170898365871023197e1 * t1647 * t5012 + 0.65854491829355115987e0 * t19566 * t1090 - 0.13170898365871023197e1 * t19569 * t4999 + 0.13170898365871023197e1 * t4981 * t19573 - 0.65854491829355115987e0 * t4996 * t19576 + 0.65854491829355115987e0 * t12127 * t19580 + 0.13170898365871023197e1 * t1087 * t19584 + 0.65854491829355115987e0 * t3278 * t6383 + 0.13170898365871023197e1 * t16381 * t1689 + 0.13170898365871023197e1 * t4743 * t1692 - 0.26341796731742046394e1 * t12122 * t19594 + 0.13170898365871023197e1 * t12127 * t19597 + 0.13170898365871023197e1 * t4954 * t5009 + 0.26341796731742046394e1 * t19603 * t4984;
    (t19606,)
}
