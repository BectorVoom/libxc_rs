//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1036/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1036(t3246: f64, t914: f64, t1245: f64, t2393: f64, t1250: f64, t2436: f64, t2439: f64, t2443: f64, t2448: f64, t3259: f64, t3260: f64, t3266: f64, t3269: f64, t3270: f64, t3273: f64, t397: f64, t6566: f64, t6574: f64, t8480: f64, t8507: f64, t8508: f64, t8512: f64, t8516: f64, t8519: f64, t8520: f64, t8529: f64, t8533: f64, t8536: f64, t8539: f64, t8542: f64, t8543: f64, t8546: f64, t943: f64, t946: f64) -> (f64, f64) {
    let t8549 = t914 * t3246;
    let t8554 = t2393 * t1245;
    let t8559 = 0.39512695097613069591e1_f64 * t8507 * t8508 + 0.26341796731742046394e1_f64 * t8512 * t3260 + 0.26341796731742046394e1_f64 * t8516 * t3260 - 0.39512695097613069591e1_f64 * t8519 * t8520 + 0.13170898365871023197e1_f64 * t3259 * t6566 + 0.65854491829355115987e0_f64 * t6574 * t1250 + 0.13170898365871023197e1_f64 * t2439 * t3266 - 0.13170898365871023197e1_f64 * t8529 * t3270 + 0.65854491829355115987e0_f64 * t943 * t8533 - 0.13170898365871023197e1_f64 * t8536 * t3270 - 0.65854491829355115987e0_f64 * t3269 * t8539 + 0.65854491829355115987e0_f64 * t8542 * t8543 + 0.13170898365871023197e1_f64 * t8546 * t2436 + 0.13170898365871023197e1_f64 * t8549 * t946 + 0.65854491829355115987e0_f64 * t3273 * t2443 - 0.65854491829355115987e0_f64 * t8554 * t2448 + 0.65854491829355115987e0_f64 * t397 * t8480;
    (t8549, t8559)
}
