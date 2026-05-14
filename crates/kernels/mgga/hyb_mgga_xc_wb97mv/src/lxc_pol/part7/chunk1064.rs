//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1064/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1064<F: Float>(t11009: F, t11011: F, t11016: F, t11020: F, t11023: F, t11027: F, t11031: F, t6817: F, t6891: F, t8958: F, t9039: F, t9040: F, t11056: F, t828: F, t10963: F, t10978: F, t10985: F, t10987: F, t10990: F, t10996: F, t11003: F, t11005: F, t6762: F, t6798: F, t8908: F, t8947: F) -> (F, F, F) {
    let t11066 = 0.31558125e0 * t11009 + 0.6311625e0 * t11011 - t6891 + 0.34731666666666666666e0 * t6817 + 0.69463333333333333333e0 * t8958 - t9039 - t9040 - 0.20839e0 * t11016 + 0.62517e0 * t11020 - 0.20839e0 * t11023 + 0.312585e0 * t11027 + 0.312585e0 * t11031;
    let t11067 = t11056 + t11066;
    let t11068 = t11067 * t828;
    let t11085 = 0.19419375e1 * t10985 - 0.258925e1 * t10987 - 0.1294625e1 * t10990 + 0.258925e1 * t10996 - t6798 + 0.40256666666666666667e0 * t6762 + 0.80513333333333333333e0 * t8908 - t8947 - 0.301925e0 * t10963 + 0.905775e0 * t10978 - 0.412621875e-1 * t11003 + 0.16504875e0 * t11005;
    (t11067, t11068, t11085)
}
