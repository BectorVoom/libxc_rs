//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1288/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1288<F: Float>(t11179: F, t112266: F, t112283: F, t112286: F, t116336: F, t116340: F, t116344: F, t116351: F, t116354: F, t116357: F, t2063: F, t32893: F, t32913: F, t33031: F, t33059: F, t34023: F, t34073: F, t34125: F, t5032: F) -> (F,) {
    let t116359 = 0.69444444444444444446e-2 * t112266 * t34023 + 0.10416666666666666667e-1 * t34073 * t32893 - 0.46296296296296296298e-2 * t112283 - 0.69444444444444444446e-2 * t33031 * t11179 * t33059 * t2063 * t5032 - 0.22109259259259259258e-2 * t116336 + 0.23148148148148148148e-2 * t112286 - 0.33163888888888888888e-2 * t116340 - 0.16581944444444444444e-2 * t116344 - 0.27777777777777777779e-1 * t34125 * t32913 - 0.27777777777777777779e-1 * t34125 * t32893 - 0.23148148148148148149e-2 * t116351 + 0.27636574074074074073e-2 * t116354 + 0.73697530864197530861e-2 * t116357;
    (t116359,)
}
