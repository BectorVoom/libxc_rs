//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 980/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk980<F: Float>(t6276: F, t829: F, t4546: F, t3210: F, t3200: F, t4555: F, t4554: F, t2861: F, t6488: F, t6493: F, t13192: F, t4549: F, t1747: F, t2840: F, t1017: F, t86: F) -> (F, F, F, F, F, F, F) {
    let t18508 = t6276 * t829;
    let t18509 = t4546 * t18508;
    let t18510 = t3210 * t18509;
    let t18511 = t3200 * t18510;
    let t18513 = t4555 * t18508;
    let t18514 = t3210 * t18513;
    let t18515 = t4554 * t18514;
    let t18517 = t2861 * t6488;
    let t18521 = t2861 * t6493;
    let t18523 = t13192 * t4549;
    let t18525 = t2840 * t1747;
    let t18527 = t86 * t1017 * t18525;
    (t18508, t18511, t18515, t18517, t18521, t18523, t18527)
}
