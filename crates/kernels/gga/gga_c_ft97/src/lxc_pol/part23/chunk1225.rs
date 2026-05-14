//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1225/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1225<F: Float>(t5029: F, t70: F, t27574: F, t30779: F, t30781: F, t27521: F, t30839: F, t108940: F, t109002: F, t109017: F, t1103: F, t122782: F, t122787: F, t123074: F, t123145: F, t123177: F, t13519: F, t13520: F, t17807: F, t17819: F, t18128: F, t24361: F, t2446: F, t25057: F, t27527: F, t27552: F, t27557: F, t27605: F, t27669: F, t27670: F, t27672: F, t27711: F, t27712: F, t3773: F, t3817: F, t52594: F, t6023: F, t6027: F, t6034: F, t6035: F, t6036: F, t6037: F, t66076: F, t66384: F, t66565: F, t79542: F, t79641: F, t79855: F, t9: F, t96692: F) -> (F,) {
    let t123278 = t5029 * t70;
    let t123304 = t30779 * t27574 * t30781;
    let t123321 = t27521 * t27574 * t30839;
    let t123327 = -0.76612330055555555556e-1 * t108940 + 0.88910709717637694816e-2 * t27711 * t25057 * t27712 * t3817 + 0.22270151833971792333e-3 * t6034 * t6035 * t6036 * t18128 + 0.21775259570994641392e-2 * t6034 * t123278 * t6037 - 0.25537443351851851852e-1 * t24361 * t6035 * t2446 * t123145 + 0.10338048737805743098e-3 * t27527 * t6023 * t123177 - 0.24041029937711879617e-5 * t79641 * t3773 * t6027 * t66384 - 0.51690243689028715488e-4 * t13520 * t6023 * t122782 - 0.51690243689028715488e-4 * t13520 * t6023 * t122787 + 0.12020514968855939808e-5 * t17819 * t13519 * t6027 * t66565 - 0.60548059007656442387e-3 * t123304 - 0.12255510004984495842e-5 * t66076 * t27605 * t27552 - 0.24511020009968991684e-6 * t17807 * t27605 * t27557 + 0.27620809331261011349e-4 * t27670 * t9 * t1103 * t27672 + 0.31073410497668637766e-5 * t79542 * t27669 * t123074 + 0.17024962234567901235e-1 * t109002 - t109017 + 0.60548059007656442387e-3 * t123321 + 0.51690243689028715487e-4 * t52594 * t6023 * t79855 - 0.49489226297715094074e-4 * t96692;
    (t123327,)
}
