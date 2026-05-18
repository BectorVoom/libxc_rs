//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 452/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk452<F: Float>(t1403: F, t1764: F, t571: F, t11: F, t1407: F, t572: F, t1755: F, t1756: F, t1761: F, t173: F, t184: F, t199: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1765 = t1764 * t1403;
    let t1766 = t571 * t1765;
    let t1767 = t11 * t1766;
    let t1769 = t572 * t1407;
    let t1770 = t571 * t1769;
    let t1771 = t11 * t1770;
    let t1773 = -t1755 - F::new(0.12594444444444444445e-2) * t1756 + F::new(0.12594444444444444445e-2) * t1761 - F::new(0.37783333333333333334e-2) * t1767 + F::new(0.18891666666666666667e-2) * t1771;
    let t1774 = t173 * t1773;
    let t1775 = t1774 * t184;
    let t1777 = F::new(2.0) / F::new(15.0) * t1775 * t199;
    (t1765, t1766, t1767, t1769, t1770, t1771, t1773, t1774, t1775, t1777)
}
