//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 953/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk953<F: Float>(t1060: F, t12703: F, t13153: F, t13220: F, t144: F, t17198: F, t1901: F, t20035: F, t20902: F, t20930: F, t2179: F, t2185: F, t3439: F, t4454: F, t4458: F, t446: F, t4462: F, t4668: F, t4724: F, t4839: F, t569: F, t77383: F, t77386: F, t86973: F, t87163: F, t925: F) -> (F,) {
    let t87657 = -2.0 / 3.0 * t446 * t569 * t4839 * t4462 + 8.0 * t446 * t2185 * t2179 * t4668 * t4724 + 4.0 / 3.0 * t446 * t569 * t4839 * t4458 - 8.0 / 3.0 * t446 * t569 * t1060 * t20035 - t446 * t144 * t87163 / 3.0 - 4.0 / 3.0 * t77383 - 8.0 / 3.0 * t1901 * t12703 * t86973 - 8.0 / 3.0 * t1901 * t13220 * t20902 * t925 + 4.0 / 9.0 * t1901 * t3439 * t17198 * t4454 - 8.0 / 3.0 * t1901 * t13153 * t20930 + 4.0 / 9.0 * t77386;
    (t87657,)
}
