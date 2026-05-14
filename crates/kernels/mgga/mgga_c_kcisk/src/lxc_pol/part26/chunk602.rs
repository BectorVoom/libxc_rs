//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 602/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk602<F: Float>(t1341: F, t5967: F, t1415: F, t1411: F, t2233: F, t3739: F, t1220: F, t3740: F, t3749: F, t3774: F, t3930: F, t5604: F, t5608: F, t5610: F, t5614: F, t5617: F, t5623: F, t5629: F, t5637: F, t5870: F, t5875: F, t5880: F, t5883: F, t5888: F) -> (F, F, F, F, F) {
    let t5968 = t1341 * t5967;
    let t5969 = t1415 * t5968;
    let t5970 = t1411 * t5969;
    let t5972 = t3739 * t2233;
    let t5974 = 0.33163888888888888888e-2 * t5604 + 0.16581944444444444444e-2 * t5608 + 0.11054629629629629629e-2 * t5610 - 0.44218518518518518517e-2 * t5614 - 0.66327777777777777776e-2 * t5617 - 0.16581944444444444444e-2 * t3740 + 0.11054629629629629629e-2 * t5623 - 0.33163888888888888888e-2 * t5629 + 0.27636574074074074073e-2 * t5637 + 0.24872916666666666666e-2 * t5870 + 0.11054629629629629629e-2 * t3749 + 0.11054629629629629629e-2 * t3774 + 0.193e0 * t1220 * t5875 + 0.74498e-1 * t3930 * t5875 + 0.16581944444444444444e-2 * t5880 + 0.16581944444444444444e-2 * t5883 - 0.24872916666666666666e-2 * t5888 - 0.24872916666666666666e-2 * t5970 - 0.16581944444444444444e-2 * t5972;
    (t5968, t5969, t5970, t5972, t5974)
}
