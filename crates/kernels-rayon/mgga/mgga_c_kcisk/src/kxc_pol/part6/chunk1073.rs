//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1073/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1073(t524: f64, t31492: f64, t31613: f64, t1589: f64, t1586: f64, t2326: f64, t8335: f64, t14963: f64, t1580: f64, t21621: f64, t21675: f64, t2318: f64, t2328: f64, t27710: f64, t27778: f64, t27791: f64, t27796: f64, t27810: f64, t31466: f64, t31470: f64, t31474: f64, t31479: f64, t31484: f64, t535: f64, t6459: f64, t8308: f64, t8324: f64, t8337: f64, t8400: f64) -> (f64, f64) {
    let t536 = 0.0_f64 < t524;
    let t31614 = t31492 + t31613;
    let t31616 = piecewise3(t536, t31614, -t31614);
    let t31617 = t1589 * t31616;
    let t31618 = t1586 * t31617;
    let t31625 = t8335 * t2326;
    let t31626 = t14963 * t31625;
    let t31627 = t1586 * t31626;
    let t31638 = 0.53972366148531951639e-1_f64 * t1580 * t31466 - 0.35981577432354634427e-1_f64 * t1580 * t31470 + 0.35981577432354634428e-1_f64 * t1580 * t31474 - 0.2698618307426597582e-1_f64 * t1580 * t31479 - 0.2698618307426597582e-1_f64 * t1580 * t31484 - 0.53972366148531951639e-1_f64 * t6459 * t8324 - 0.17990788716177317214e-1_f64 * t27710 - 0.2698618307426597582e-1_f64 * t535 * t31618 - 0.8095854922279792746e-1_f64 * t2318 * t8400 - 0.8095854922279792746e-1_f64 * t8308 * t2328 - 0.16191709844559585492e0_f64 * t535 * t31627 + 0.16191709844559585492e0_f64 * t2318 * t8337 - 0.17990788716177317214e-1_f64 * t27778 - 0.2698618307426597582e-1_f64 * t27791 + 0.53972366148531951639e-1_f64 * t27796 - 0.53972366148531951639e-1_f64 * t27810 - 0.59969295720591057378e-2_f64 * t21621 + 0.17990788716177317213e-1_f64 * t21675;
    (t31614, t31638)
}
