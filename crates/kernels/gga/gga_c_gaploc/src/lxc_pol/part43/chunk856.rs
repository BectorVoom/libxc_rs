//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 856/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk856<F: Float>(t2487: F, t41726: F, t6711: F, t10532: F, t10533: F, t41749: F, t41810: F, t6716: F, t6717: F, t41965: F, t6914: F, t10557: F, t30936: F) -> (F, F, F, F, F, F) {
    let t42233 = F::new(0.14953741122029092374e3) * t2487 * t6711 * t41726;
    let t42236 = F::new(0.55213813373645879534e2) * t10532 * t10533 * t41749;
    let t42239 = F::new(0.69017266717057349418e1) * t6716 * t6717 * t41810;
    let t42242 = F::new(0.62115540045351614476e2) * t6914 * t6717 * t41965;
    let t42245 = F::new(0.27606906686822939767e2) * t10532 * t10533 * t41965;
    let t42250 = F::new(0.17875244975925213335e2) * t10557 * t30936;
    (t42233, t42236, t42239, t42242, t42245, t42250)
}
