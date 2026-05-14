//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1295/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1295<F: Float>(t74846: F, t9648: F, t1799: F, t32903: F, t6668: F, t5054: F, t6676: F, t112586: F, t16576: F, t5182: F, t1755: F, t2788: F, t16581: F, t18325: F, t34121: F, t34191: F) -> (F, F, F, F, F, F, F) {
    let t116489 = t9648 * t74846;
    let t116495 = t1799 * t32903 * t6668;
    let t116498 = t5054 * t32903 * t6676;
    let t116507 = t5182 * t112586 * t16576;
    let t116509 = t2788 * t1755;
    let t116511 = t5182 * t116509 * t16581;
    let t116513 = t34121 * t18325;
    let t116516 = t34191 * t18325;
    (t116489, t116495, t116498, t116507, t116511, t116513, t116516)
}
