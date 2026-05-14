//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1063/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1063<F: Float>(t10051: F, t754: F, t10050: F, t257: F, t255: F, t235: F, t9680: F, t677: F, t9682: F, t675: F, t9568: F, t683: F, t7514: F, t191: F, t33300: F, t2371: F, t2404: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41402 = t754 * t10051;
    let t41408 = 1.0 / t10050 / t257;
    let t41409 = t255 * t41408;
    let t41547 = 1.0 / t9680 / t235;
    let t41593 = t677 * t9682;
    let t41816 = t9568 * t675;
    let t41825 = t683 * t7514;
    let t41848 = t191 * t33300;
    let t41879 = t2404 * t2371;
    (t41402, t41408, t41409, t41547, t41593, t41816, t41825, t41848, t41879)
}
