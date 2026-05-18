//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1247/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1247<F: Float>(t16455: F, t32889: F, t7585: F, t10820: F, t22693: F, t7427: F, t10930: F, t5750: F, t579: F, t23344: F, t7573: F, t2628: F, t8516: F) -> (F, F, F, F, F) {
    let t33101 = F::new(0.23005755572352449806e2) * t16455 * t7585 * t32889;
    let t33105 = F::new(0.1656414401209376386e3) * t7427 * t22693 * t10820;
    let t33109 = F::new(0.73618417831527839379e2) * t10930 * t579 * t5750 * t10820;
    let t33112 = F::new(0.13803453343411469884e2) * t23344 * t7573 * t32889;
    let t33113 = t8516 * t2628;
    (t33101, t33105, t33109, t33112, t33113)
}
