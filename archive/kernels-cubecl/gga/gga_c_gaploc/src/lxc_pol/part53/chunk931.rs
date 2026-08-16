//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 931/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk931<F: Float>(t25070: F, t7427: F, t9438: F, t1022: F, t9641: F, t2009: F, t2021: F, t43683: F, t7584: F, t7585: F, t1445: F, t3234: F, t813: F, t8528: F) -> (F, F, F, F) {
    let t43930 = t7427 * t9438 * t25070;
    let t43931 = F::cast_from(0.47928657442400937096e-1_f64) * t43930;
    let t43932 = t9641 * t1022;
    let t43935 = F::cast_from(0.35750489951850426669e0_f64) * t2021 * t43932 * t2009;
    let t43938 = F::cast_from(0.11502877786176224903e2_f64) * t7584 * t7585 * t43683;
    let t43955 = F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t8528 * t3234;
    (t43931, t43935, t43938, t43955)
}
