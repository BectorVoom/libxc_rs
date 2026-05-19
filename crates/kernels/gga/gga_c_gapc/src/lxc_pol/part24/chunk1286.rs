//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1286/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1286<F: Float>(t35720: F, t35722: F, t35725: F, t35727: F, t35732: F, t35736: F, t35738: F, t35741: F, t35745: F, t35747: F, t35749: F, t35753: F, t35755: F) -> F {
    let t37510 = F::cast_from(0.58714905980103539484e-5_f64) * t35720 - F::cast_from(0.93943849568165663176e-4_f64) * t35722 - F::cast_from(0.29357452990051769742e-5_f64) * t35725 + F::cast_from(0.68394856556563412154e-6_f64) * t35727 + F::cast_from(0.39896999657995323756e-6_f64) * t35732 + F::cast_from(0.13678971311312682431e-5_f64) * t35736 + F::cast_from(0.68394856556563412154e-6_f64) * t35738 - F::cast_from(0.19571635326701179828e-6_f64) * t35741 + F::cast_from(0.25687771366295298524e-5_f64) * t35745 - F::cast_from(0.54107930964747943838e-4_f64) * t35747 + F::cast_from(0.12311074180181414188e-4_f64) * t35749 + F::cast_from(0.16703216453219854913e-4_f64) * t35753 + F::cast_from(0.37902149675095557569e-5_f64) * t35755;
    t37510
}
