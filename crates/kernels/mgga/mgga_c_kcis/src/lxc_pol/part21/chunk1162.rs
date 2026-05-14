//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1162/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1162<F: Float>(t1014: F, t27940: F, t3245: F, t8051: F, t15573: F, t2173: F, t27918: F, t26784: F, t26823: F, t27812: F, t27895: F, t27915: F, t7696: F, t8034: F, t93143: F, t93145: F, t93606: F, t93610: F, t93628: F, t95769: F) -> (F, F, F) {
    let t96270 = t1014 * t27940;
    let t96273 = t3245 * t8051;
    let t96281 = 0.46336805555555555556e-3 * t2173 * t15573 * t27918;
    let t96286 = 0.30891203703703703704e-3 * t93606 + 0.1621345679012345679e-1 * t93143 + 0.23168402777777777778e-3 * t93610 - 0.88437037037037037034e-2 * t96270 - 0.11054629629629629629e-2 * t93145 - 0.55273148148148148147e-3 * t96273 + t93628 - 0.2782641015625e-3 * t27895 * t26784 - 0.37069444444444444444e-2 * t7696 * t27915 + t96281 + 0.69505208333333333333e-3 * t26823 * t8034 - 0.37134344353515625e-4 * t27812 * t95769;
    (t96270, t96273, t96286)
}
