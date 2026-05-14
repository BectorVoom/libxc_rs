//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 749/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk749<F: Float>(t158: F, t3460: F, t1054: F, t1790: F, t183: F, t3410: F, t1034: F, t1044: F, t164: F, t167: F, t1717: F, t1721: F, t3441: F, t588: F, t626: F, t1045: F, t1055: F, t184: F, t188: F) -> (F, F, F, F, F, F) {
    let t3461 = t3460 * t158;
    let t3466 = t1054 * t1054;
    let t3467 = t1790 * t3466;
    let t3470 = t183 * t3410;
    let t3487 = 0.13170898365871023197e1 * t1717 * t3470 * t1721 - 0.13170898365871023197e1 * t588 * t1044 * t1034 * t164 - 0.65854491829355115987e0 * t588 * t183 * t3441 * t164 - 0.65854491829355115987e0 * t588 * t3470 * t164 + 0.65854491829355115987e0 * t167 * t3460;
    let t3488 = t626 * t3487;
    let t3491 = 0.65854491829355115987e0 * t3461 * t188 - 0.13170898365871023197e1 * t1045 * t1055 + 0.13170898365871023197e1 * t184 * t3467 - 0.65854491829355115987e0 * t184 * t3488;
    (t3461, t3466, t3467, t3487, t3488, t3491)
}
