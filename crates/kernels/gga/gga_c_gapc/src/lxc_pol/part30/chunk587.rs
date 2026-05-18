//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 587/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk587<F: Float>(t1112: F, t687: F, t2970: F, t2976: F, t2984: F, t2988: F, t2991: F, t3002: F, t3009: F, t3014: F, t3019: F, t3023: F, t3025: F) -> (F, F) {
    let t3483 = t1112 * t687;
    let t3497 = F::new(0.10821235962619981449e-3) * t2970 + F::new(0.12163329537032409896e-2) * t2976 - F::new(0.20241536458333333335e-4) * t2984 + F::new(0.17376185052903442709e-3) * t2988 + F::new(0.17376185052903442709e-3) * t2991 + F::new(0.16882592796244404291e-6) * t3002 + F::new(0.33765185592488808582e-6) * t3009 - F::new(0.50680539737635041235e-4) * t3014 - F::new(0.14492726735651760868e-5) * t3019 + F::new(0.28985453471303521736e-5) * t3023 - F::new(0.16908181191593721013e-4) * t3025;
    (t3483, t3497)
}
