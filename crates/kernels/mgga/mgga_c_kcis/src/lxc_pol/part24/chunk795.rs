//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 795/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk795<F: Float>(t1022: F, t9409: F, t2861: F, t4774: F, t4549: F, t9429: F, t4802: F, t4820: F, t4825: F, t10338: F, t1754: F, t2943: F, t304: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14072 = t9409 * t1022;
    let t14078 = t2861 * t4774;
    let t14079 = F::new(0.33163888888888888888e-2) * t14078;
    let t14081 = t9429 * t4549;
    let t14085 = t9429 * t4802;
    let t14086 = F::new(0.22109259259259259258e-2) * t14085;
    let t14102 = t2861 * t4820;
    let t14103 = F::new(0.66327777777777777776e-2) * t14102;
    let t14104 = t2861 * t4825;
    let t14115 = t10338 * t1754;
    let t14117 = t304 * t2943;
    (t14072, t14078, t14079, t14081, t14085, t14086, t14102, t14103, t14104, t14115, t14117)
}
