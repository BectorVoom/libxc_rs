//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1101/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1101<F: Float>(t32897: F, t6066: F, t6111: F, t10811: F, t7772: F, t2976: F, t7503: F, t10820: F, t818: F, t825: F, t22706: F, t2684: F, t28099: F, t24390: F, t955: F, t7366: F, t8775: F) -> (F, F, F, F, F, F, F, F) {
    let t32900 = 0.85801175884441024006e1 * t6111 * t6066 * t32897;
    let t32902 = 0.17875244975925213335e2 * t10811 * t7772;
    let t32903 = t2976 * t7503;
    let t32904 = 0.89376224879626066674e-1 * t32903;
    let t32907 = 0.24539472610509279794e2 * t825 * t818 * t10820;
    let t32910 = 0.11656249489991907902e3 * t2684 * t22706 * t10820;
    let t32911 = 0.15976219147466979032e-1 * t28099;
    let t32923 = 0.47667319935800568892e0 * t955 * t24390;
    let t32925 = 0.23833659967900284446e0 * t8775 * t7366;
    (t32900, t32902, t32904, t32907, t32910, t32911, t32923, t32925)
}
