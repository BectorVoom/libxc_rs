//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1223/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1223<F: Float>(t2004: F, t3856: F, t1175: F, t8184: F, t19: F, t2003: F, t3968: F, t10214: F, t10245: F, t10273: F, t10296: F, t1173: F, t1224: F, t125: F, t1829: F, t1861: F, t1877: F, t1967: F, t1971: F, t1993: F, t1996: F, t21662: F, t21670: F, t25362: F, t25367: F, t26: F, t29: F, t29160: F, t29635: F, t2987: F, t2989: F, t3003: F, t544: F, t554: F, t556: F, t557: F, t641: F, t669: F) -> (F,) {
    let t29641 = t3856 * t2004;
    let t29643 = t1175 * t8184;
    let t29646 = t19 * t2003 * t3968;
    let t29663 = -3.0 / 32.0 * t10296 * t641 - 3.0 / 32.0 * t10296 * t669 - 3.0 / 64.0 * t3856 * t1967 - 3.0 / 32.0 * t3856 * t1971 - 3.0 / 64.0 * t3856 * t1829 - t554 * t557 * t10245 * t1877 / 64.0 - t1993 * t1996 * t10245 * t1861 / 48.0 - t554 * t557 * t25362 * t1173 / 16.0 - t554 * t557 * t25367 * t1173 / 32.0 - t29160 / 72.0 - 3.0 / 64.0 * t19 * t26 * t29 * t29635 * t125 + t29641 / 96.0 + t29643 / 48.0 + t29646 / 96.0 - t554 * t3003 * t556 * t1224 * t125 / 16.0 + t21662 / 96.0 + t21670 - t2987 * t2989 * t125 * t10273 * t544 / 24.0 - t2987 * t2989 * t10214 * t1877 / 48.0;
    (t29663,)
}
