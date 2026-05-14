//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 663/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk663<F: Float>(t10933: F, t11032: F, t10944: F, t10947: F, t10951: F, t10954: F, t10960: F, t10966: F, t11038: F, t11052: F, t11054: F, t11057: F, t11060: F, t11063: F, t11084: F, t1737: F, t1746: F) -> (F,) {
    let t11091 = 0.93932222222222222223e0 * t10933;
    let t11092 = 0.73586666666666666667e0 * t11032;
    let t11099 = -0.60385000000000000001e0 * t10944 + 0.30192500000000000001e0 * t10947 - 0.33547222222222222222e0 * t10951 + 0.12077e1 * t10954 - 0.181155e1 * t10960 - 0.301925e0 * t10966 - t11091 - t11092 + 0.19419375e1 * t11038 + 0.16504875e0 * t11052 + 0.258925e1 * t11054 - 0.412621875e-1 * t11057 + 0.247573125e0 * t11060 - 0.3883875e1 * t11063;
    let t11100 = t11084 + t11099;
    let t11102 = t1737 * t11100 * t1746;
    (t11102,)
}
