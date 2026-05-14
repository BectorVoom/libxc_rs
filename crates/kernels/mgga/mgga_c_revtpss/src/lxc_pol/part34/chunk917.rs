//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 917/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk917<F: Float>(t1066: F, t23485: F, t247: F, t1651: F, t5819: F, t4801: F, t1042: F, t1668: F, t6305: F, t373: F, t11257: F, t11506: F, t23451: F, t11509: F, t981: F, t23448: F, t23450: F, t23461: F, t23463: F, t23465: F, t23469: F, t23549: F, t23552: F, t23554: F, t23556: F) -> (F, F, F, F, F, F, F, F) {
    let t23630 = t247 * t1066 * t23485;
    let t23633 = t5819 * t1651;
    let t23634 = t4801 * t23633;
    let t23635 = t1042 * t23634;
    let t23640 = t6305 * t1668;
    let t23641 = t373 * t23640;
    let t23642 = t23641 * t11257;
    let t23643 = t1042 * t23642;
    let t23648 = t11506 * t23451;
    let t23649 = t23648 * t11509;
    let t23651 = 0.10254018858216406658e4 * t981 * t23649;
    let t23652 = t23461 + t23463 + t23465 - t23469 + t23549 + t23552 - t23651 + t23448 - t23554 - t23556 - t23450;
    (t23630, t23633, t23635, t23640, t23641, t23643, t23651, t23652)
}
