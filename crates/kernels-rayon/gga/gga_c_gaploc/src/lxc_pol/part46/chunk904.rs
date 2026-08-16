//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 904/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk904(t123: f64, t31730: f64, t2326: f64, t9074: f64, t12797: f64, t1358: f64, t12773: f64, t6305: f64, t2268: f64, t42212: f64, t888: f64, t1365: f64, t42408: f64, t42625: f64, t42629: f64, t42633: f64, t42637: f64, t42638: f64, t42641: f64, t42645: f64, t42648: f64, t42652: f64, t42655: f64, t42659: f64, t42661: f64, t42664: f64) -> f64 {
    let t42669 = t31730 * t123;
    let t42671 = t9074 * t42669 * t2326;
    let t42673 = t1358 * t12797;
    let t42674 = 0.31616674039640166221e-2_f64 * t42673;
    let t42675 = t6305 * t12773;
    let t42678 = t2268 * t42212 * t888;
    let t42680 = -0.1138200265427045984e0_f64 * t42625 - t42629 - t42633 + t42637 - t42638 + t42641 - t42645 + t42648 - t42652 + t42655 - t42659 - 0.23712505529730124666e-2_f64 * t42661 + 0.23712505529730124666e-2_f64 * t42664 + 0.31616674039640166221e-2_f64 * t1358 * t1365 * t42408 - 0.71137516589190373998e-2_f64 * t42671 - t42674 - 0.1707300398140568976e0_f64 * t42675 - 0.1707300398140568976e0_f64 * t42678;
    t42680
}
