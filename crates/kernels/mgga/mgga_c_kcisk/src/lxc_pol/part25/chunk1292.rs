//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1292/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1292<F: Float>(t112266: F, t112289: F, t116406: F, t116409: F, t116413: F, t116416: F, t116423: F, t116426: F, t17337: F, t2063: F, t32893: F, t33005: F, t33031: F, t33059: F, t34013: F, t34032: F, t34225: F, t5015: F, t5038: F, t68280: F, t9652: F, t9672: F) -> (F,) {
    let t116443 = 0.73697530864197530862e-3 * t116406 - 0.23280625000000000001e-2 * t116409 * t33005 + 0.8041666666666666667e-2 * t116413 * t9652 - 0.55555555555555555558e-1 * t116416 * t9672 - 0.10722222222222222223e-1 * t34225 * t32893 - 0.55555555555555555558e-1 * t116416 * t9652 + 0.22109259259259259258e-2 * t116423 + 0.46296296296296296297e-2 * t116426 + 0.69444444444444444446e-2 * t112266 * t34032 + 0.69444444444444444446e-2 * t112289 * t34032 + 0.34722222222222222223e-2 * t33031 * t5015 * t33059 * t2063 * t5038 - 0.13888888888888888889e-1 * t33031 * t17337 * t33059 * t68280 + 0.69444444444444444446e-2 * t112266 * t34013;
    (t116443,)
}
