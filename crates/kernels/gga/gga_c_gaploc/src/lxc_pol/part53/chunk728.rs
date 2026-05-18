//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 728/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk728<F: Float>(t14458: F, t14463: F, t14472: F, t14477: F, t502: F, t12605: F, t12609: F, t13184: F, t13187: F, t13193: F, t13195: F, t13919: F, t13922: F, t13925: F, t13935: F, t13938: F) -> (F, F, F) {
    let t14479 = t14458 + t14463 + t14472 + t14477;
    let t14480 = t502 * t14479;
    let t14489 = F::new(0.15381052460284448567e-1) * t13919 - F::new(0.64087718584518535698e-3) * t13935 + F::new(0.30762104920568897134e-1) * t13922 + t13184 - t13187 + F::new(0.64087718584518535698e-3) * t13938 - F::new(0.46143157380853345702e-1) * t13925 + t13193 + F::new(0.1281754371690370714e-2) * t13195 - F::new(0.19226315575355560709e-2) * t12605 + F::new(0.12817543716903707139e-2) * t12609;
    (t14479, t14480, t14489)
}
