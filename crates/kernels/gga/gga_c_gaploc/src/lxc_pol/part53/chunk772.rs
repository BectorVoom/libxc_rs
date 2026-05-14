//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 772/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk772<F: Float>(t43027: F, t1841: F, t3487: F, t6118: F, t9755: F, t10809: F, t161: F, t9744: F, t7064: F, t7069: F, t8878: F, t13212: F, t7129: F, t40693: F, t40696: F, t40699: F) -> (F, F, F, F, F, F, F, F) {
    let t43028 = 0.64087718584518535698e-3 * t43027;
    let t43032 = 0.59815204012217299984e-2 * t1841 * t9755 * t3487 * t6118;
    let t43040 = 0.10254034973522965711e-1 * t1841 * t10809 * t161 * t9744;
    let t43042 = t7064 * t8878 * t7069;
    let t43043 = 0.1922631557535556071e-2 * t43042;
    let t43049 = 0.23071578690426672851e-1 * t7129 * t13212;
    let t43053 = 0.64087718584518535698e-3 * t40693;
    let t43054 = 0.64087718584518535698e-3 * t40696;
    let t43055 = 0.64087718584518535698e-3 * t40699;
    (t43028, t43032, t43040, t43043, t43049, t43053, t43054, t43055)
}
