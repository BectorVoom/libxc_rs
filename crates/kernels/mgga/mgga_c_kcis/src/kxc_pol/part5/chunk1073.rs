//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1073/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1073<F: Float>(t15022: F, t18648: F, t18657: F, t5134: F, t18574: F, t5142: F, t18677: F, t18672: F, t1018: F, t1745: F, t4581: F, t2840: F, t4567: F, t19536: F, t304: F, t238: F, t6676: F, t86: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19980 = t15022 * t18648;
    let t19983 = t5134 * t18657;
    let t19986 = t5142 * t18574;
    let t19989 = t5142 * t18677;
    let t19992 = t5134 * t18672;
    let t19995 = t1018 * t1745;
    let t19996 = t19995 * t4581;
    let t19999 = t2840 * t1745;
    let t20000 = t19999 * t4567;
    let t20003 = t304 * t19536;
    let t20008 = t86 * t238 * t6676;
    (t19980, t19983, t19986, t19989, t19992, t19996, t20000, t20003, t20008)
}
