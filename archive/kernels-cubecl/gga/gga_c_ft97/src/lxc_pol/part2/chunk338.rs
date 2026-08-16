//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 338/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk338<F: Float>(t1557: F, t1736: F, t1559: F, t420: F, t419: F, t1570: F, t422: F, t1580: F, t423: F, t1718: F, t1722: F, t1726: F, t1731: F, t1733: F) -> (F, F, F, F, F, F, F) {
    let t1737 = t1736 * t1557;
    let t1738 = t1737 * t1559;
    let t1739 = t420 * t1738;
    let t1740 = t419 * t1739;
    let t1742 = t422 * t1570;
    let t1743 = t1742 * t1559;
    let t1744 = t420 * t1743;
    let t1745 = t419 * t1744;
    let t1747 = t423 * t1580;
    let t1748 = t420 * t1747;
    let t1749 = t419 * t1748;
    let t1751 = F::cast_from(0.18727458458024691358e0_f64) * t1718 - F::cast_from(0.3404992446913580247e-1_f64) * t1722 - F::cast_from(0.3404992446913580247e-1_f64) * t1726 - t1731 + F::cast_from(0.42562405586419753086e-2_f64) * t1733 + F::cast_from(0.85124811172839506173e-2_f64) * t1740 - F::cast_from(0.12768721675925925926e-1_f64) * t1745 + F::cast_from(0.6384360837962962963e-2_f64) * t1749;
    (t1738, t1740, t1743, t1745, t1747, t1749, t1751)
}
