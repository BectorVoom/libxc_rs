//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1135/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1135<F: Float>(t100999: F, t101098: F, t101295: F, t115397: F, t115820: F, t115885: F, t115944: F, t115956: F, t115970: F, t115973: F, t115977: F, t115981: F, t115985: F, t1300: F, t15636: F, t15829: F, t15879: F, t1701: F, t1737: F, t22522: F, t22541: F, t22826: F, t25708: F, t409: F, t45573: F, t5538: F, t5546: F, t5570: F, t5611: F, t92644: F, t92715: F, t938: F) -> (F,) {
    let t115990 = -0.85124811172839506173e-2 * t25708 * t115944 - 0.17024962234567901235e-1 * t22541 * t5570 * t1737 * t115885 + 0.17024962234567901235e-1 * t22522 * t5570 * t1737 * t115820 - 0.14846767889314528222e-4 * t115956 + 0.46509801892875584e-1 * t22826 * t15829 + 0.46509801892875584e-1 * t22826 * t15636 - 0.27039520901431665705e-3 * t45573 * t409 * t100999 * t938 + 0.22227677429409423704e-2 * t1300 * t1701 * t5546 * t15879 - 0.10357803499222879255e-4 * t115970 * t101098 - 0.10357803499222879255e-4 * t115973 * t92644 - 0.34049924469135802469e-1 * t25708 * t115977 - 0.76612330055555555556e-1 * t101295 + 0.22699949646090534979e-1 * t5611 * t115981 - 0.28374937057613168724e-2 * t115985 + 0.28677218675336554254e-7 * t5538 * t92715 * t115397;
    (t115990,)
}
