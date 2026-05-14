//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1014/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1014<F: Float>(t18648: F, t9714: F, t26: F, t18657: F, t2970: F, t4714: F, t18685: F, t939: F, t18570: F, t945: F, t18574: F, t18677: F, t18672: F, t6383: F, t659: F, t6386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18908 = t9714 * t18648;
    let t18909 = t26 * t18908;
    let t18911 = t2970 * t18657;
    let t18912 = t4714 * t18911;
    let t18920 = t939 * t18685;
    let t18923 = t945 * t18570;
    let t18924 = t26 * t18923;
    let t18926 = t945 * t18574;
    let t18927 = t4714 * t18926;
    let t18929 = t945 * t18677;
    let t18930 = t26 * t18929;
    let t18932 = t2970 * t18672;
    let t18933 = t26 * t18932;
    let t18935 = t659 * t6383;
    let t18937 = t659 * t6386;
    (t18909, t18912, t18920, t18924, t18927, t18930, t18933, t18935, t18937)
}
