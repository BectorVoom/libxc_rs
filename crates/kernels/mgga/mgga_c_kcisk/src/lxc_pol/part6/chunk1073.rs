//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1073/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1073<F: Float>(t524: F, t31492: F, t31613: F, t1589: F, t1586: F, t2326: F, t8335: F, t14963: F, t1580: F, t21621: F, t21675: F, t2318: F, t2328: F, t27710: F, t27778: F, t27791: F, t27796: F, t27810: F, t31466: F, t31470: F, t31474: F, t31479: F, t31484: F, t535: F, t6459: F, t8308: F, t8324: F, t8337: F, t8400: F) -> (F, F) {
    let t536 = F::new(0.0) < t524;
    let t31614 = t31492 + t31613;
    let t31616 = piecewise3::<f64>(t536, t31614, -t31614);
    let t31617 = t1589 * t31616;
    let t31618 = t1586 * t31617;
    let t31625 = t8335 * t2326;
    let t31626 = t14963 * t31625;
    let t31627 = t1586 * t31626;
    let t31638 = F::new(0.53972366148531951639e-1) * t1580 * t31466 - F::new(0.35981577432354634427e-1) * t1580 * t31470 + F::new(0.35981577432354634428e-1) * t1580 * t31474 - F::new(0.2698618307426597582e-1) * t1580 * t31479 - F::new(0.2698618307426597582e-1) * t1580 * t31484 - F::new(0.53972366148531951639e-1) * t6459 * t8324 - F::new(0.17990788716177317214e-1) * t27710 - F::new(0.2698618307426597582e-1) * t535 * t31618 - F::new(0.8095854922279792746e-1) * t2318 * t8400 - F::new(0.8095854922279792746e-1) * t8308 * t2328 - F::new(0.16191709844559585492e0) * t535 * t31627 + F::new(0.16191709844559585492e0) * t2318 * t8337 - F::new(0.17990788716177317214e-1) * t27778 - F::new(0.2698618307426597582e-1) * t27791 + F::new(0.53972366148531951639e-1) * t27796 - F::new(0.53972366148531951639e-1) * t27810 - F::new(0.59969295720591057378e-2) * t21621 + F::new(0.17990788716177317213e-1) * t21675;
    (t31614, t31638)
}
