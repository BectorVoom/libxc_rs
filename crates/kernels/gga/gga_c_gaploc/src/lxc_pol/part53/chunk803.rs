//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 803/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk803<F: Float>(t2009: F, t2021: F, t43932: F, t43683: F, t7584: F, t7585: F, t1445: F, t3234: F, t813: F, t8528: F, t2949: F, t9688: F, t13130: F, t2194: F, t13157: F, t4673: F, t6060: F) -> (F, F, F, F, F, F) {
    let t43935 = 0.35750489951850426669e0 * t2021 * t43932 * t2009;
    let t43938 = 0.11502877786176224903e2 * t7584 * t7585 * t43683;
    let t43955 = 0.46011511144704899612e1 * t813 * t1445 * t8528 * t3234;
    let t43959 = 0.46011511144704899612e1 * t813 * t1445 * t2949 * t9688;
    let t43961 = 0.46011511144704899612e1 * t2194 * t13130;
    let t43972 = 0.14300195980740170667e1 * t6060 * t4673 * t13157;
    (t43935, t43938, t43955, t43959, t43961, t43972)
}
