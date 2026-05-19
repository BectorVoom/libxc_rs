//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1113/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1113<F: Float>(t10007: F, t1880: F, t7394: F, t9438: F, t1944: F, t3240: F, t3248: F, t1949: F, t731: F, t9625: F, t21455: F, t739: F) -> (F, F, F, F, F, F) {
    let t29078 = t7394 * t9438 * t10007 * t1880;
    let t29160 = F::cast_from(0.19938401337405766662e-2_f64) * t1944 * t3240;
    let t29162 = F::cast_from(0.19938401337405766662e-2_f64) * t1944 * t3248;
    let t29184 = F::cast_from(0.17090058289204942853e-2_f64) * t1949 * t3248;
    let t29186 = F::cast_from(0.17090058289204942853e-2_f64) * t731 * t9625;
    let t29190 = t739 * t21455;
    (t29078, t29160, t29162, t29184, t29186, t29190)
}
