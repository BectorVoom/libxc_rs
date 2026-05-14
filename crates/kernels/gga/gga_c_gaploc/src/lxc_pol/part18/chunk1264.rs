//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1264/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1264<F: Float>(t34068: F, t34112: F, t34160: F, t34207: F, t34254: F, t34271: F, t34315: F, t34346: F, t34384: F, t34426: F, t34450: F, t34493: F, t34534: F, t34549: F, t34589: F, t34615: F, t34656: F, t34703: F, t34756: F, t34784: F, t34832: F, t34870: F, t34898: F, t34923: F, t34948: F, t34980: F, t35025: F, t35067: F, t35117: F, t35151: F, t35189: F, t35233: F, t502: F) -> (F,) {
    let t35239 = t502 * (t35233 + t35189 + t35151 + t35117 + t35067 + t35025 + t34980 + t34948 + t34923 + t34898 + t34870 + t34832 + t34784 + t34756 + t34703 + t34656 + t34615 + t34589 + t34549 + t34534 + t34493 + t34450 + t34426 + t34384 + t34346 + t34315 + t34271 + t34254 + t34207 + t34160 + t34112 + t34068);
    (t35239,)
}
