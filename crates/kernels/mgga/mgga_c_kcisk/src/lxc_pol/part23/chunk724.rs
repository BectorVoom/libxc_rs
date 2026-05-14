//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 724/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk724<F: Float>(t1459: F, t6394: F, t3740: F, t3749: F, t3774: F, t5604: F, t5608: F, t5610: F, t5614: F, t5617: F, t5623: F, t5629: F, t5637: F, t5870: F, t5880: F, t5883: F, t5888: F, t5970: F, t5972: F, t5977: F, t5979: F) -> (F, F) {
    let t6395 = t1459 * t6394;
    let t6415 = 0.23214722222222222222e-2 * t5604 + 0.11607361111111111111e-2 * t5608 + 0.77382407407407407407e-3 * t5610 - 0.30952962962962962963e-2 * t5614 - 0.46429444444444444443e-2 * t5617 - 0.11607361111111111111e-2 * t3740 + 0.77382407407407407407e-3 * t5623 - 0.23214722222222222222e-2 * t5629 + 0.19345601851851851852e-2 * t5637 + 0.17411041666666666666e-2 * t5870 + 0.77382407407407407407e-3 * t3749 + 0.77382407407407407407e-3 * t3774 + 0.11607361111111111111e-2 * t5880 + 0.11607361111111111111e-2 * t5883 - 0.17411041666666666666e-2 * t5888 - 0.17411041666666666666e-2 * t5970 - 0.11607361111111111111e-2 * t5972 + 0.46429444444444444443e-2 * t5977 + 0.77382407407407407407e-3 * t5979;
    (t6395, t6415)
}
