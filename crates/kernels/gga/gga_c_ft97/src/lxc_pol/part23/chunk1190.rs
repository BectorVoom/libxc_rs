//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1190/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1190<F: Float>(t109758: F, t6749: F, t24237: F, t30875: F, t1403: F, t30905: F, t681: F, t10157: F, t107756: F, t107919: F, t107920: F, t107971: F, t107979: F, t1137: F, t18459: F, t18514: F, t193: F, t24191: F, t27947: F, t28018: F, t28036: F, t28037: F, t28461: F, t30909: F, t30911: F, t3837: F, t5996: F, t6002: F, t6745: F, t98195: F) -> (F,) {
    let t121849 = t109758 * t6749;
    let t121851 = t24237 * t30875;
    let t121856 = t1403 * t681 * t30905;
    let t121875 = -t5996 * t30911 / 3.0 - t1403 * t193 * t24191 * t30909 / 3.0 + t6745 * t27947 / 3.0 + t121849 / 27.0 + t121851 / 81.0 - 2.0 * t1137 * t28461 + t107971 + 2.0 / 9.0 * t121856 - 2.0 / 27.0 * t107979 + 2.0 * t6002 * t10157 * t28018 * t3837 + 2.0 / 27.0 * t6002 * t98195 * t28037 * t18459 + 2.0 / 9.0 * t6002 * t28036 * t107756 * t18514 - 5.0 / 81.0 * t6002 * t107919 * t107920 * t18514;
    (t121875,)
}
