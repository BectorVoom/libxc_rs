//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1095/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1095<F: Float>(t10810: F, t870: F, t10684: F, t10648: F, t10958: F, t10971: F, t10962: F, t37599: F, t37822: F, t37833: F, t37919: F, t38001: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38355 = t870 * t10810;
    let t38356 = t38355 * t10684;
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38452 = F::new(0.42952285777298855708e-4) * t37599;
    let t38528 = F::new(0.14224135994914204065e1) * t37822;
    let t38532 = F::new(0.17888640988868435534e-2) * t37833;
    let t38568 = F::new(0.18496169001454677638e1) * t37919;
    let t38597 = F::new(0.31806003678208078381e-2) * t38001;
    (t38355, t38356, t38359, t38362, t38452, t38528, t38532, t38568, t38597)
}
