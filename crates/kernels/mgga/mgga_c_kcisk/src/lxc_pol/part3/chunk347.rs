//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 347/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk347<F: Float>(t1724: F, t1725: F, t1695: F, t1699: F, t45: F, t625: F, t630: F, t1718: F, t1710: F, t1715: F, t1722: F) -> (F, F, F, F, F, F) {
    let t1726 = t1724 * t1725;
    let t1729 = F::new(0.92708333333333333333e-2) * t1695;
    let t1731 = -t1729 - F::new(0.92708333333333333333e-2) * t1699;
    let t1735 = t45 * t625;
    let t1736 = t630 * t630;
    let t1737 = F::new(1.0) / t1736;
    let t1739 = F::new(0.301925e0) * t1695;
    let t1742 = F::new(0.16557e0) * t1718;
    let t1744 = F::new(0.258925e1) * t1710 - t1739 - F::new(0.301925e0) * t1699 + F::new(0.16504875e0) * t1715 - t1742 - F::new(0.16557e0) * t1722;
    (t1726, t1731, t1735, t1736, t1737, t1744)
}
