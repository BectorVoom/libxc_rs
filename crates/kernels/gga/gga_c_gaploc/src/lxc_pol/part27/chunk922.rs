//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 922/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk922<F: Float>(t1967: F, t9889: F, t7810: F, t7403: F, t959: F, t7340: F, t3281: F, t5676: F, t2530: F, t2610: F, t2365: F, t2033: F) -> (F, F, F, F, F, F, F, F) {
    let t9890 = t1967 * t9889;
    let t9891 = t7810 * t9890;
    let t9935 = F::cast_from(0.29792074959875355558e-1_f64) * t7403 * t959;
    let t9937 = F::cast_from(0.29792074959875355558e-1_f64) * t7340 * t959;
    let t9942 = F::cast_from(0.29792074959875355558e-1_f64) * t5676 * t3281;
    let t9943 = t2610 * t2530;
    let t9944 = t2365 * t9943;
    let t9946 = F::cast_from(0.29792074959875355558e-1_f64) * t2033 * t9944;
    (t9890, t9891, t9935, t9937, t9942, t9943, t9944, t9946)
}
