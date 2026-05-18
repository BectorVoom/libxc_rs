//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1284/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1284<F: Float>(t10820: F, t22693: F, t7427: F, t10930: F, t5750: F, t579: F, t23344: F, t32889: F, t7573: F, t2628: F, t8516: F, t2684: F, t32948: F, t7585: F) -> (F, F, F, F, F) {
    let t33105 = F::new(0.1656414401209376386e3) * t7427 * t22693 * t10820;
    let t33109 = F::new(0.73618417831527839379e2) * t10930 * t579 * t5750 * t10820;
    let t33112 = F::new(0.13803453343411469884e2) * t23344 * t7573 * t32889;
    let t33113 = t8516 * t2628;
    let t33114 = F::new(0.59584149919750711116e-1) * t33113;
    let t33117 = F::new(0.87421871174939309262e2) * t2684 * t7585 * t32948;
    (t33105, t33109, t33112, t33114, t33117)
}
