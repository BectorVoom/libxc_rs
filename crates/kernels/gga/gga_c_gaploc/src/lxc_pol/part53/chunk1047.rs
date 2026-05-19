//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1047/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1047<F: Float>(t40890: F, t40896: F, t43243: F, t43254: F, t43257: F, t43260: F, t43263: F, t43265: F, t43267: F, t43269: F, t43274: F, t43282: F, t43286: F, t47723: F, t47731: F, t47734: F, t47737: F, t47740: F) -> F {
    let t51047 = F::cast_from(0.30762104920568897134e-1_f64) * t47723 - t43243 + t43254 + t43257 + F::cast_from(0.64087718584518535698e-3_f64) * t47731 + t43260 + t43263 + t43265 - t43267 - t43269 - F::cast_from(0.46143157380853345702e-1_f64) * t47734 - F::cast_from(0.46143157380853345702e-1_f64) * t47737 - F::cast_from(0.46143157380853345702e-1_f64) * t47740 - t43274 + F::cast_from(0.25635087433807414279e-2_f64) * t40890 - t43282 - F::cast_from(0.17090058289204942852e-2_f64) * t40896 - t43286;
    t51047
}
