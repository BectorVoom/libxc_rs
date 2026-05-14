//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 446/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk446<F: Float>(t5828: F, t814: F, t1730: F, t820: F, t316: F, t5814: F, t101: F, t1580: F, t1584: F, t1711: F, t1715: F, t1721: F, t309: F, t317: F, t3901: F, t4861: F, t4882: F, t544: F, t5800: F, t5804: F, t5810: F, t5815: F, t5825: F, t87: F, t98: F) -> (F,) {
    let t5829 = t5828 * t814;
    let t5832 = t820 * t1730;
    let t5833 = t5832 * t316;
    let t5836 = -t5814;
    let t5837 = t101 * t5836;
    let t5840 = -100.0 / 27.0 * t309 * t1711 - 20.0 / 27.0 * t87 * t5800 + 40.0 / 9.0 * t4861 * t5804 - 50.0 / 9.0 * t309 * t1715 + 20.0 / 9.0 * t87 * t5810 + 10.0 / 3.0 * t87 * t5815 + 400.0 / 27.0 * t1721 * t317 - 200.0 / 27.0 * t544 * t1580 + 100.0 / 9.0 * t544 * t1584 - 20.0 / 27.0 * t98 * t5825 - 40.0 / 9.0 * t4882 * t5829 + 20.0 / 9.0 * t98 * t5833 + 10.0 / 3.0 * t98 * t5837 + t3901;
    (t5840,)
}
