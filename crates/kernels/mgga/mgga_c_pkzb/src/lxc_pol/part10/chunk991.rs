//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 991/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk991<F: Float>(t7930: F, t6090: F, t6093: F, t6180: F, t6183: F, t6211: F, t7947: F, t7950: F, t7955: F, t7959: F, t7961: F, t7967: F, t7979: F, t7982: F, t6177: F, t6218: F, t7970: F, t7973: F, t7975: F, t7986: F, t7990: F, t7994: F, t7997: F, t8000: F) -> (F, F, F, F, F) {
    let t8076 = 0.60385e0 * t7930;
    let t8085 = -t6211 + 0.80513333333333333334e0 * t6090 - 0.301925e0 * t6093 - t8076 + 0.905775e0 * t7947 + 0.27595e0 * t7950 + 0.258925e1 * t7959 + 0.16504875e0 * t7961 - 0.16557e0 * t6180 - 0.16557e0 * t6183 + 0.40256666666666666667e0 * t7955 - 0.258925e1 * t7967;
    let t8090 = 0.33114e0 * t7979;
    let t8091 = 0.33114e0 * t7982;
    let t8097 = -0.1294625e1 * t7970 + 0.16504875e0 * t7973 + 0.82524375e-1 * t7975 - t6218 + 0.5519e0 * t6177 - t8090 - t8091 + 0.248355e0 * t7986 + 0.49671e0 * t7990 + 0.248355e0 * t7994 + 0.19419375e1 * t7997 - 0.412621875e-1 * t8000;
    (t8076, t8085, t8090, t8091, t8097)
}
