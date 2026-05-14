//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1444/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1444<F: Float>(t2625: F, t2858: F, t9589: F, t1048: F, t2850: F, t8601: F, t288: F, t9880: F, t481: F, t19712: F, t20180: F, t23199: F, t25041: F, t31514: F, t32999: F, t33000: F, t33001: F, t33002: F, t33003: F) -> (F, F, F, F) {
    let t34900 = 18.0 * t2858 * t9589 * t2625;
    let t34903 = 3.0 * t1048 * t8601 * t2850;
    let t34904 = t288 * t9880;
    let t34907 = 6.0 * t2858 * t34904 * t481;
    let t34909 = -t25041 + t19712 + t34900 + t34903 - t34907 + t23199 + t32999 + t20180 + t33000 - 0.7089e1 * t31514 + t33001 + t33002 - t33003;
    (t34900, t34903, t34907, t34909)
}
