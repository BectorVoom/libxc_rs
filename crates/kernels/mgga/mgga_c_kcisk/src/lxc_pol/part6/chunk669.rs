//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 669/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk669<F: Float>(t13522: F, t119: F, t416: F, t1337: F, t142: F, t10: F, t3529: F, t1265: F, t4125: F, t373: F, t4128: F, t357: F, t4079: F, t346: F, t1311: F, t163: F) -> (F, F, F, F, F, F, F, F) {
    let t13523 = 0.55403703703703703703e-1 * t13522;
    let t13524 = t119 * t416;
    let t13528 = t142 * t1337;
    let t13538 = t10 * t3529;
    let t13561 = 1.0 / t4125 / t1265;
    let t13565 = 1.0 / t4128 / t373;
    let t13587 = 1.0 / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13603 = t163 * t1311;
    (t13523, t13524, t13528, t13538, t13561, t13565, t13588, t13603)
}
