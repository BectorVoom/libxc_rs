//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1191/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1191<F: Float>(t3255: F, t7218: F, t1889: F, t3761: F, t5481: F, t1897: F, t5477: F, t16025: F, t5489: F, t1098: F, t7246: F, t7234: F, t1102: F, t11384: F, t11632: F, t16401: F, t16408: F, t16410: F, t16436: F, t16441: F, t16543: F, t21453: F, t22044: F, t22049: F, t22055: F, t22060: F, t22064: F, t22067: F, t4587: F, t486: F) -> (F,) {
    let t22069 = t3255 * t7218;
    let t22072 = t3761 * t1889 * t5481;
    let t22076 = t3761 * t5477 * t1897;
    let t22079 = t16025 * t5489;
    let t22082 = t1098 * t7246;
    let t22085 = t1098 * t7234;
    let t22089 = 0.10950716111111111111e-2 * t1102 * t22044 + 0.492782225e-3 * t1102 * t22049 + 0.43802864444444444443e-3 * t16401 + 0.7391733375e-3 * t1102 * t22055 - 0.1478346675e-2 * t1102 * t22060 - 0.295669335e-2 * t1102 * t22064 - 0.87605728888888888887e-3 * t22067 - t16408 + t16410 + 0.13140859333333333333e-2 * t22069 + 0.19711289e-2 * t1102 * t22072 - 0.39422578e-2 * t4587 * t22076 - 0.19711289e-2 * t11632 * t22079 + 0.13140859333333333333e-2 * t22082 + t16436 - 0.65704296666666666667e-3 * t16441 + 0.492782225e-3 * t22085 - 4.0 * t486 * t21453 + t11384 + t16543;
    (t22089,)
}
