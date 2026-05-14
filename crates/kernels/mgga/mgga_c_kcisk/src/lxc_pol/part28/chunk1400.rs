//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1400/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1400<F: Float>(t4581: F, t9066: F, t24251: F, t9704: F, t1930: F, t9055: F, t9086: F, t117294: F, t7317: F, t1800: F, t24199: F, t34313: F, t7307: F, t7299: F, t24146: F, t117419: F, t7333: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t122190 = t4581 * t9066;
    let t122193 = t9704 * t24251;
    let t122195 = t1930 * t9055;
    let t122197 = t4581 * t9086;
    let t122199 = t117294 * t7317;
    let t122201 = t1800 * t24199;
    let t122203 = t34313 * t7307;
    let t122205 = t34313 * t7299;
    let t122207 = t9704 * t24146;
    let t122209 = t117419 * t7333;
    (t122190, t122193, t122195, t122197, t122199, t122201, t122203, t122205, t122207, t122209)
}
