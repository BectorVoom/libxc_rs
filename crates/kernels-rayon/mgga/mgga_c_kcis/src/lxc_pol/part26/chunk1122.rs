//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1122/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1122(t28613: f64, t28643: f64, t1506: f64, t1628: f64, t8236: f64, t1636: f64, t17710: f64, t18268: f64, t2268: f64, t27702: f64, t28562: f64, t28563: f64, t28564: f64, t28566: f64, t28567: f64, t28569: f64, t28572: f64, t28575: f64, t28578: f64, t28582: f64, t6225: f64, t6256: f64, t7998: f64, t8001: f64) -> (f64, f64, f64, f64) {
    let t28644 = t28613 + t28643;
    let t28645 = t1506 * t28644;
    let t28649 = t8236 * t1628;
    let t28654 = -t1636 * t28649 - t17710 * t2268 + 2.0_f64 * t18268 * t8001 + 2.0_f64 * t27702 * t6225 - t6256 * t7998 - t28562 + t28563 + t28564 - t28566 + t28567 - t28569 + t28572 - t28575 - t28578 - t28582;
    (t28644, t28645, t28649, t28654)
}
