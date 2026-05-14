//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1313/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1313<F: Float>(t19862: F, t6353: F, t31963: F, t6213: F, t6374: F, t72231: F, t2843: F, t31640: F, t875: F, t1466: F, t31673: F, t681: F, t31668: F, t10683: F, t111732: F, t111815: F, t112402: F, t1218: F, t18997: F, t193: F, t19362: F, t28985: F, t29410: F, t31672: F, t4027: F, t4162: F, t5207: F, t6216: F, t6217: F, t6391: F, t7129: F, t880: F) -> (F, F, F, F) {
    let t125696 = t6353 * t19862;
    let t125698 = t31963 * t6213;
    let t125702 = t72231 * t6374;
    let t125705 = t2843 * t31640 * t875;
    let t125711 = t1466 * t681 * t31673;
    let t125714 = t1466 * t681 * t31668;
    let t125729 = t112402 - t5207 * t6391 - 2.0 * t125696 - t125698 / 18.0 - 2.0 * t4027 * t7129 + 4.0 * t125702 + 4.0 * t125705 - 2.0 / 3.0 * t111732 * t111815 * t18997 - t125711 / 18.0 - t125714 / 9.0 + t1466 * t193 * t31672 * t880 / 6.0 + t6216 * t10683 * t6217 * t19362 + 2.0 * t6216 * t10683 * t28985 * t4162 - 2.0 * t1218 * t29410;
    (t125696, t125702, t125705, t125729)
}
