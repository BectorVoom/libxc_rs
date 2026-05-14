//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1225/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1225<F: Float>(t34587: F, t10215: F, t10495: F, t1339: F, t1359: F, t1424: F, t1430: F, t1537: F, t30788: F, t30791: F, t30794: F, t34556: F, t34558: F, t34566: F, t34567: F, t34573: F, t34576: F, t34579: F, t34581: F, t34583: F, t34586: F, t544: F, t590: F, t6716: F, t6717: F) -> (F,) {
    let t34588 = 0.17875244975925213335e0 * t34587;
    let t34589 = -0.1022478025437886658e1 * t1537 * t1339 * t10215 * t590 + t34556 + 0.23833659967900284446e0 * t34558 * t1430 - 0.79445533226334281486e-1 * t544 * t1359 * t10495 * t1424 + t30788 + t30791 - t30794 - t34566 + 0.13803453343411469884e2 * t6716 * t6717 * t34567 - t34573 + t34576 + t34579 + t34581 - t34583 - t34586 + t34588;
    (t34589,)
}
