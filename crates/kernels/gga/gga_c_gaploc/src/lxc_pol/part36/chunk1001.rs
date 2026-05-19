//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1001/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1001<F: Float>(t43925: F, t10867: F, t41511: F, t25070: F, t7427: F, t9438: F, t1022: F, t9641: F, t2009: F, t2021: F, t43683: F, t7584: F, t7585: F) -> (F, F, F, F, F) {
    let t43926 = F::cast_from(0.89376224879626066675e-1_f64) * t43925;
    let t43927 = t10867 * t41511;
    let t43928 = F::cast_from(0.89376224879626066675e-1_f64) * t43927;
    let t43930 = t7427 * t9438 * t25070;
    let t43931 = F::cast_from(0.47928657442400937096e-1_f64) * t43930;
    let t43932 = t9641 * t1022;
    let t43935 = F::cast_from(0.35750489951850426669e0_f64) * t2021 * t43932 * t2009;
    let t43938 = F::cast_from(0.11502877786176224903e2_f64) * t7584 * t7585 * t43683;
    (t43926, t43928, t43931, t43935, t43938)
}
