//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1050/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1050<F: Float>(t11329: F, t9262: F, t27063: F, t3709: F, t26017: F, t19771: F, t3718: F, t436: F, t8775: F, t8776: F, t34940: F, t34942: F, t34946: F, t34949: F, t34951: F, t34954: F) -> (F,) {
    let t34956 = t11329 * t9262;
    let t34958 = t3709 * t27063;
    let t34960 = t3709 * t26017;
    let t34962 = t3718 * t19771;
    let t34965 = t8775 * t436 * t8776;
    let t34967 = 0.80045999977926802213e-7 * t34940 - 0.20259111355493285149e-5 * t34942 + 0.88397049170382309323e-8 * t34946 - 0.90579542097823505428e-7 * t34949 - 0.25301920572916666668e-5 * t34951 - 0.49190053374354708085e-8 * t34954 - 0.13259557375557346398e-6 * t34956 - 0.13259557375557346398e-6 * t34958 - 0.6629778687778673199e-7 * t34960 - 0.90579542097823505428e-7 * t34962 - 0.22510123728325872388e-7 * t34965;
    (t34967,)
}
