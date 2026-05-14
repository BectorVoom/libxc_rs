//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 968/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk968<F: Float>(t7832: F, t8451: F, t1245: F, t2363: F, t3246: F, t914: F, t2393: F, t1250: F, t2436: F, t2439: F, t2443: F, t2448: F, t3259: F, t3260: F, t3266: F, t3269: F, t3270: F, t3273: F, t397: F, t6566: F, t6574: F, t8480: F, t8507: F, t8508: F, t8512: F, t8516: F, t8519: F, t8520: F, t8529: F, t8533: F, t8536: F, t8539: F, t8542: F, t943: F, t946: F) -> (F, F, F) {
    let t8543 = t7832 * t8451;
    let t8546 = t2363 * t1245;
    let t8549 = t914 * t3246;
    let t8554 = t2393 * t1245;
    let t8559 = 0.39512695097613069591e1 * t8507 * t8508 + 0.26341796731742046394e1 * t8512 * t3260 + 0.26341796731742046394e1 * t8516 * t3260 - 0.39512695097613069591e1 * t8519 * t8520 + 0.13170898365871023197e1 * t3259 * t6566 + 0.65854491829355115987e0 * t6574 * t1250 + 0.13170898365871023197e1 * t2439 * t3266 - 0.13170898365871023197e1 * t8529 * t3270 + 0.65854491829355115987e0 * t943 * t8533 - 0.13170898365871023197e1 * t8536 * t3270 - 0.65854491829355115987e0 * t3269 * t8539 + 0.65854491829355115987e0 * t8542 * t8543 + 0.13170898365871023197e1 * t8546 * t2436 + 0.13170898365871023197e1 * t8549 * t946 + 0.65854491829355115987e0 * t3273 * t2443 - 0.65854491829355115987e0 * t8554 * t2448 + 0.65854491829355115987e0 * t397 * t8480;
    (t8543, t8549, t8559)
}
