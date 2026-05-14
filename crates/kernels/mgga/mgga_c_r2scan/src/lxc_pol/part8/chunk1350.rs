//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1350/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1350<F: Float>(t109: F, t1541: F, t20200: F, t2504: F, t2505: F, t2526: F, t3016: F, t32266: F, t481: F, t486: F, t490: F, t7175: F, t7180: F, t7184: F, t8629: F, t8675: F, t8688: F, t9115: F, t915: F, t9880: F, t9937: F, t9938: F, t9941: F, t9944: F) -> (F,) {
    let t33061 = -12.0 * t1541 * t2504 * t481 * t9880 - 360.0 * t20200 * t2504 * t481 * t9937 + 3.0 * t109 * t32266 * t490 - 36.0 * t2504 * t2505 * t8629 + 180.0 * t2504 * t2526 * t8675 - 36.0 * t2504 * t3016 * t7184 + 180.0 * t2504 * t7180 * t9115 + 60.0 * t486 * t9938 + 3.0 * t486 * t9944 - 36.0 * t7175 * t9941 + 9.0 * t8688 * t915;
    (t33061,)
}
